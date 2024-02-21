use adw::subclass::prelude::*;
use adw::TabPage;
use glib::{clone, Properties, SignalHandlerId};
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use tracing::*;

use crate::components::{PreferencesWindow, TerminalTab};
use crate::config::PROFILE;
use crate::settings::Settings;
use crate::twl::{StyleSwitcher, ZoomControls};
use crate::util::EnvMap;

#[derive(Debug, gtk::CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/window.ui")]
#[properties(wrapper_type=super::Window)]
pub struct Window {
    pub settings: Settings,

    /// The initial working directory for a new terminal
    #[property(get, set, construct, nullable)]
    directory: RefCell<Option<PathBuf>>,

    /// The foreground command for a new terminal
    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    /// The initial env for a new terminal
    #[property(set, get, construct, nullable)]
    env: RefCell<Option<EnvMap>>,

    #[property(get, set, construct, default = 100)]
    pub zoom_level: Cell<u32>,

    #[template_child]
    pub title_widget: TemplateChild<adw::WindowTitle>,

    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,

    #[template_child]
    pub header_box: TemplateChild<gtk::Box>,

    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,

    #[template_child]
    pub style_switcher: TemplateChild<StyleSwitcher>,

    #[template_child]
    pub toasts: TemplateChild<adw::ToastOverlay>,

    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,

    #[template_child]
    pub tab_overview: TemplateChild<adw::TabOverview>,

    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,

    #[template_child]
    pub toolbar_view: TemplateChild<adw::ToolbarView>,

    selected_page_signals: glib::SignalGroup,
    active_tab_signals: glib::SignalGroup,
    active_tab_bindings: glib::BindingGroup,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            directory: Default::default(),
            command: Default::default(),
            env: Default::default(),
            header_bar: Default::default(),
            toasts: Default::default(),
            tab_view: Default::default(),
            tab_bar: Default::default(),
            header_box: Default::default(),
            title_widget: Default::default(),
            menu_button: Default::default(),
            style_switcher: Default::default(),
            zoom_level: Default::default(),
            toolbar_view: Default::default(),
            tab_overview: Default::default(),

            selected_page_signals: glib::SignalGroup::new::<adw::TabPage>(),
            active_tab_signals: glib::SignalGroup::new::<TerminalTab>(),
            active_tab_bindings: glib::BindingGroup::new(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TermsWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        StyleSwitcher::ensure_type();
        ZoomControls::ensure_type();
        klass.bind_template();
        klass.bind_template_callbacks();

        klass.install_action("zoom.out", None, move |win: &super::Window, _, _| {
            win.imp().zoom_out();
        });
        klass.install_action("zoom.reset", None, move |win: &super::Window, _, _| {
            win.imp().zoom_reset();
        });
        klass.install_action("zoom.in", None, move |win: &super::Window, _, _| {
            win.imp().zoom_in();
        });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        if PROFILE.should_use_devel_class() {
            let obj = self.obj();
            obj.add_css_class("devel");
        }

        self.setup_signals();
        self.setup_widgets();
        self.setup_gactions();
        self.connect_signals();
    }

    fn dispose(&self) {
        self.tab_bar.unparent();
        self.title_widget.unparent();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

#[gtk::template_callbacks]
impl Window {
    fn setup_signals(&self) {
        self.selected_page_signals.connect_bind_local(clone!(@weak self as this => move |_sg, obj| {
            info!("selected page: bind");
            let page = obj.downcast_ref::<adw::TabPage>();
            let tab_obj = page.map(adw::TabPage::child);
            let tab = tab_obj.and_downcast_ref::<TerminalTab>();
            this.active_tab_signals.set_target(tab);
            this.active_tab_bindings.set_source(tab);
        }));

        self.active_tab_signals.connect_bind_local(move |sg, obj| {
            info!("active tab: bind");
        });

        self.selected_page_signals.connect_notify_local(Some("pinned"), move |obj, param| {
            info!("selected page: pinned");
        });

        self.active_tab_bindings.bind("title", self.obj().as_ref(), "title").sync_create().build();
        self.active_tab_bindings
            .bind("directory", self.obj().as_ref(), "directory")
            .sync_create()
            .build();

        // self.active_tab_signals.connect_local(Some("bell"), move |sg, obj| {
        //     info!("active tab: bind");
        // });

        self.active_tab_bindings.connect_notify_local(Some("zoom"), move |obj, param| {
            info!("active tab: zoom");
        });

        self.settings.bind_show_menu_button(&*self.menu_button, "visible").get_only().build();

        self.obj()
            .bind_property("fullscreened", &*self.header_bar, "show-end-title-buttons")
            .invert_boolean()
            .sync_create()
            .build();

        self.settings.bind_style_preference(&*self.style_switcher, "preference").build();

        self.settings
            .bind_show_headerbar(&*self.toolbar_view, "extend-content-to-top-edge")
            .get_only()
            .invert_boolean()
            .build();
        self.settings.bind_show_headerbar(&*self.toolbar_view, "reveal-top-bars").get_only().build();

        self.set_integrated_tab_bar();

        self.settings
            .connect_headerbar_integrated_tabbar_changed(clone!(@weak self as this => move |_| {
                this.set_integrated_tab_bar();
            }));

        self.tab_bar.connect_view_notify(clone!(@weak self as this => move |tabbar| {
            this.set_integrated_tab_bar();
            if let Some(tab_view) = tabbar.view() {
                info!("tab view: {:?}", tab_view);
                tab_view.connect_n_pages_notify( move |_| {
                    this.set_integrated_tab_bar();
                });
            }
        }));
    }

    fn setup_widgets(&self) {
        if self.settings.remember_window_size() {
            self.restore_window_size();
        }
    }

    fn restore_window_size(&self) {
        let obj = self.obj();
        obj.set_default_width(self.settings.window_width() as i32);
        obj.set_default_height(self.settings.window_height() as i32);
        obj.set_fullscreened(self.settings.was_fullscreened());
        obj.set_maximized(self.settings.was_maximized());
    }

    fn connect_signals(&self) {
        self.obj().connect_default_width_notify(clone!(@weak self as this => move |w| {
            this.settings.set_window_width(w.default_width() as u32);
        }));

        self.obj().connect_default_height_notify(clone!(@weak self as this => move |w| {
            this.settings.set_window_height(w.default_height() as u32);
        }));

        self.obj().connect_fullscreened_notify(clone!(@weak self as this => move |w| {
            this.settings.set_was_fullscreened(w.is_fullscreened());
        }));

        self.obj().connect_maximized_notify(clone!(@weak self as this => move |w| {
            this.settings.set_was_maximized(w.is_maximized());
        }));

        // self.obj().connect_fullscreened_notify(clone!(@weak self as this => move |w| {
        //     this.header_bar.set_fullscreened(w.is_fullscreened());
        // }));

        self.tab_view.connect_n_pages_notify(clone!(@weak self as this => move |tv | {
            let n_pages = tv.n_pages();
            info!("tab_view.n_pages: {:?}", n_pages);
            if n_pages == 0 {
                this.obj().close();
            }
        }));

        self.tab_view.connect_selected_page_notify(clone!(@weak self as this => move |tab_view| {
            info!("tab_view.selected_page_notify");
            // if let Some(page) = tab_view.selected_page() {
            //     this.obj().set_title(Some(&page.title()))
            // }
        }));
    }

    fn setup_gactions(&self) {
        self.obj().add_action_entries([
            gio::ActionEntry::builder("edit-preferences")
                .activate(move |win: &super::Window, _, _| win.imp().open_preferences())
                .build(),
            gio::ActionEntry::builder("new-tab")
                .activate(move |win: &super::Window, _, _| {
                    let this = win.imp();
                    let command = this.command.borrow().clone();
                    let directory = this.directory.borrow().clone();
                    let env = this.env.borrow().clone();

                    this.new_tab(command, directory, env);
                })
                .build(),
            gio::ActionEntry::builder("toggle-fullscreen")
                .activate(move |win: &super::Window, _, _| win.set_fullscreened(!win.is_fullscreened()))
                .build(),
            gio::ActionEntry::builder("move-tab-left")
                .activate(move |win: &super::Window, _, _| win.imp().move_tab_left())
                .build(),
            gio::ActionEntry::builder("move-tab-right")
                .activate(move |win: &super::Window, _, _| win.imp().move_tab_right())
                .build(),
            gio::ActionEntry::builder("tab-overview")
                .activate(move |win: &super::Window, _, _| win.imp().open_tab_overview())
                .build(),
            gio::ActionEntry::builder("detach-tab")
                .activate(move |win: &super::Window, _, _| win.imp().detach_tab())
                .build(),
            gio::ActionEntry::builder("pin-tab")
                .activate(move |win: &super::Window, _, _| win.imp().pin_tab(true))
                .build(),
            gio::ActionEntry::builder("unpin-tab")
                .activate(move |win: &super::Window, _, _| win.imp().pin_tab(false))
                .build(),
            gio::ActionEntry::builder("rename-tab")
                .activate(move |win: &super::Window, _, _| win.imp().rename_tab())
                .build(),
            gio::ActionEntry::builder("close-tab")
                .activate(move |win: &super::Window, _, _| win.imp().close_tab())
                .build(),
            gio::ActionEntry::builder("close-other-tabs")
                .activate(move |win: &super::Window, _, _| win.imp().close_other_tabs())
                .build(),
            gio::ActionEntry::builder("add-terminal-right")
                .activate(move |win: &super::Window, _, _| win.imp().split(Some(gtk::Orientation::Horizontal)))
                .build(),
            gio::ActionEntry::builder("add-terminal-down")
                .activate(move |win: &super::Window, _, _| win.imp().split(Some(gtk::Orientation::Vertical)))
                .build(),
            gio::ActionEntry::builder("add-terminal-auto")
                .activate(move |win: &super::Window, _, _| win.imp().split(None))
                .build(),
        ]);
    }

    pub fn open_preferences(&self) {
        let prefs_window = PreferencesWindow::new(Some(self.obj().as_ref()));
        prefs_window.set_visible(true);
    }

    pub fn new_tab(&self, command: Option<String>, directory: Option<PathBuf>, env: Option<EnvMap>) -> adw::TabPage {
        let tab = TerminalTab::new(directory, command, env);
        let page = self.tab_view.append(&tab);

        self.tab_view.set_selected_page(&page);

        tab.connect_close(clone!(@weak self as this => move |tab: &TerminalTab| {
            this.tab_view.close_page(&this.tab_view.page(tab));
        }));

        tab.bind_property("title", &page, "title").sync_create().build();

        page
    }

    fn zoom_out(&self) {
        if let Some(tab) = self.tab_view.selected_page().and_then(|page| page.child().downcast::<TerminalTab>().ok()) {
            // tab.zoom_out();
        }

        // TODO

        warn!("Zoom out: not yet implemented");
    }

    fn zoom_reset(&self) {
        // TODO
        warn!("Zoom default: not yet implemented");
    }

    fn zoom_in(&self) {
        // TODO
        warn!("Zoom in: not yet implemented");
    }

    fn move_tab_left(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.reorder_backward(&page);
        }
    }

    fn move_tab_right(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.reorder_forward(&page);
        }
    }

    fn detach_tab(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            info!("Detaching tab page {:?}", page);
            if let Some(app) = self.obj().application() {
                let window = super::Window::new(&app);
                if let Some(new_tab_view) = window.tab_view() {
                    self.tab_view.transfer_page(&page, &new_tab_view, 0);
                }
                window.present();
            }
        }
    }

    fn pin_tab(&self, pinned: bool) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.set_page_pinned(&page, pinned)
        }
    }

    fn rename_tab(&self) {
        // TODO
        warn!("rename-tab: not yet implemented");
    }

    fn close_tab(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.close_page(&page);
        }
    }

    fn close_other_tabs(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.close_other_pages(&page);
        }
    }

    fn open_tab_overview(&self) {
        self.tab_overview.set_open(true)
    }

    fn split(&self, orientation: Option<gtk::Orientation>) {
        if let Some(tab) = self.tab_view.selected_page().map(|p| p.child()).and_downcast::<TerminalTab>() {
            tab.split(orientation);
        }
    }

    #[template_callback]
    fn on_selected_page_changed(&self) {
        let page = self.tab_view.selected_page();
        debug!("on page changed: {:?}", page);
        self.selected_page_signals.set_target(page.as_ref());
        if let Some(page) = page.as_ref() {
            let tab = page.child().downcast::<TerminalTab>().ok();

            if let Some(tab) = tab.as_ref() {
                tab.grab_focus();
            }
        }
    }

    #[template_callback]
    fn on_page_attached(&self) {}
    #[template_callback]
    fn on_page_detached(&self) {}

    #[template_callback]
    fn on_create_window(&self) -> Option<adw::TabView> {
        self.obj().application().and_then(|app| {
            let window = super::Window::new(&app);
            window.present();
            window.tab_view()
        })
    }
    #[template_callback]
    fn on_page_closed(&self) -> bool {
        // TODO
        false
    }
    #[template_callback]
    fn on_setup_menu(&self) {}

    #[template_callback]
    fn on_tab_overview_open(&self) {}

    #[template_callback]
    fn on_overview_create_tab(&self) -> adw::TabPage {
        // TODO: figure out if I need to check the custom command from the settings here, or all the way down?
        // Either way, it should not be the selected tab's command, that makes no sense.
        // Not even sure why that even is a property on Window at all
        self.new_tab(None, self.directory.borrow().clone(), self.env.borrow().clone())
    }

    fn set_integrated_tab_bar(&self) {
        self.tab_bar.unparent();
        if self.settings.headerbar_integrated_tabbar() {
            if self.header_bar.title_widget() != Some(self.tab_bar.clone().into()) {
                self.header_bar.set_title_widget(Some(&*self.tab_bar));
            }
            self.tab_bar.set_halign(gtk::Align::Fill);
            self.tab_bar.set_hexpand(true);
            self.tab_bar.set_autohide(false);
            self.tab_bar.set_can_focus(false);
            self.tab_bar.set_css_classes(&["inline", "integrated"]);
        } else {
            self.header_bar.set_title_widget(Some(&*self.title_widget));
            self.header_box.append(&*self.tab_bar);

            self.tab_bar.set_halign(gtk::Align::Fill);
            self.tab_bar.set_hexpand(true);
            self.tab_bar.set_autohide(true);
            self.tab_bar.set_can_focus(false);
            self.tab_bar.set_css_classes(&[]);
        }
    }
}
