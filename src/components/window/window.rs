use adw::subclass::prelude::*;
use adw::TabPage;
use glib::{clone, Properties, SignalHandlerId};
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use tracing::*;

use crate::components::PreferencesWindow;
use crate::config::PROFILE;
use crate::settings::Settings;
use crate::util::EnvMap;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
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

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub overlay: TemplateChild<gtk::Overlay>,

    #[template_child]
    pub container: TemplateChild<gtk::Box>,

    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,

    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TermsWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        // klass.install_action("tab.close-others", None, move |win: &super::Window, _, _| {
        //     win.imp().close_other_tabs();
        // });
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

        self.setup_widgets();
        self.setup_gactions();
        self.connect_signals();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

#[gtk::template_callbacks]
impl Window {
    fn setup_widgets(&self) {
        self.header_bar.set_container(Some(&*self.container));
        self.header_bar.set_overlay(Some(&*self.overlay));

        if self.settings.remember_window_size() {
            self.restore_window_size();
        }

        self.new_tab();
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

        self.obj().connect_fullscreened_notify(clone!(@weak self as this => move |w| {
            this.header_bar.set_fullscreened(w.is_fullscreened());
        }));

        self.tab_view
            .connect_close_page(clone!(@weak self as this => @default-return false, move |tv, _| {
                if tv.n_pages() <= 1 {
                    this.obj().close();
                }
                false
            }));

        self.tab_view.connect_selected_page_notify(clone!(@weak self as this => move |tab_view| {
            if let Some(page) = tab_view.selected_page() {
                this.update_title(&page);
            }

        }));
    }

    fn setup_gactions(&self) {
        self.obj().add_action_entries([
            gio::ActionEntry::builder("edit-preferences")
                .activate(move |win: &super::Window, _, _| win.imp().open_preferences())
                .build(),
            gio::ActionEntry::builder("new-tab")
                .activate(move |win: &super::Window, _, _| win.imp().new_tab())
                .build(),
            gio::ActionEntry::builder("toggle-fullscreen")
                .activate(move |win: &super::Window, _, _| win.set_fullscreened(!win.is_fullscreened()))
                .build(),
            gio::ActionEntry::builder("zoom-out")
                .activate(move |win: &super::Window, _, _| win.imp().zoom_out())
                .build(),
            gio::ActionEntry::builder("zoom-default")
                .activate(move |win: &super::Window, _, _| win.imp().zoom_default())
                .build(),
            gio::ActionEntry::builder("zoom-in")
                .activate(move |win: &super::Window, _, _| win.imp().zoom_in())
                .build(),
            gio::ActionEntry::builder("move-tab-left")
                .activate(move |win: &super::Window, _, _| win.imp().move_tab_left())
                .build(),
            gio::ActionEntry::builder("move-tab-right")
                .activate(move |win: &super::Window, _, _| win.imp().move_tab_right())
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
        ]);
    }

    pub fn open_preferences(&self) {
        let prefs_window = PreferencesWindow::new(Some(self.obj().as_ref()));
        prefs_window.set_visible(true);
    }

    pub fn new_tab(&self) {
        let command = self.command.borrow().clone();
        let directory = self.directory.borrow().clone();
        let env = self.env.borrow().clone();

        let tab = TerminalTab::new(directory, command, env);
        let page = self.tab_view.append(&tab);

        self.tab_view.set_selected_page(&page);

        tab.connect_close(clone!(@weak self as this => move |tab: &TerminalTab| {
            this.tab_view.close_page(&this.tab_view.page(tab));
        }));

        let tab_page = page.clone();
        tab.connect_title_notify(clone!(@weak self as this, @weak tab_page => move |t| {
            let term_title = t.title();
            let title = term_title.as_ref().map(String::as_str);
            if let Some(title) = title {
                tab_page.set_title(title);
            }
            this.update_title(&tab_page);
        }));

        let tab_page = page.clone();
        tab.connect_directory_notify(clone!(@weak self as this, @weak tab_page => move |t| {
            if this.tab_view.selected_page() == Some(tab_page) {
                this.update_directory(t.directory());
            }
        }));
    }

    fn update_directory(&self, directory: Option<PathBuf>) {
        *self.directory.borrow_mut() = directory;
        self.obj().notify_directory();
    }

    fn update_title(&self, page: &TabPage) {
        if self.tab_view.selected_page().as_ref() == Some(page) {
            info!("set window title {:?}", page.title());
            self.obj().set_title(Some(&page.title()))
        } else {
            info!("page does not equal selected");
        }
    }

    fn zoom_out(&self) {
        if let Some(tab) = self.tab_view.selected_page().and_then(|page| page.child().downcast::<TerminalTab>().ok()) {
            // tab.zoom_out();
        }

        // TODO

        warn!("Zoom out: not yet implemented");
    }

    fn zoom_default(&self) {
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
        // TODO
        warn!("detach tab: not yet implemented");
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
        // TODO
        warn!("Close tab: not yet implemented");
    }

    fn close_other_tabs(&self) {
        // TODO
        warn!("Close other tabs: not yet implemented");
    }
}
