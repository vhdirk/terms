use std::cell::{Cell, RefCell};

use adw::subclass::prelude::*;
use glib::clone;
use glib::{self, Properties, StaticTypeExt};
use gtk::prelude::*;
use tracing::info;

use crate::components::StyleSwitcher;
use crate::settings::Settings;
use crate::tile::ZoomControls;

#[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/header_bar.ui")]
#[properties(wrapper_type = super::HeaderBar)]
pub struct HeaderBar {
    pub settings: Settings,

    #[template_child]
    pub revealer: TemplateChild<gtk::Revealer>,

    #[template_child]
    pub title_widget: TemplateChild<adw::WindowTitle>,

    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,

    #[template_child]
    pub header_box: TemplateChild<gtk::Box>,

    #[property(get, set, construct_only)]
    pub tab_bar: RefCell<adw::TabBar>,

    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,

    #[property(get, set, nullable)]
    pub title: RefCell<Option<String>>,

    #[property(get, set)]
    pub fullscreened: Cell<bool>,

    #[property(get, set, nullable)]
    pub overlay: RefCell<Option<gtk::Overlay>>,

    #[property(get, set, nullable)]
    pub container: RefCell<Option<gtk::Box>>,
}

#[glib::object_subclass]
impl ObjectSubclass for HeaderBar {
    const NAME: &'static str = "TermsHeaderBar";
    type Type = super::HeaderBar;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        StyleSwitcher::ensure_type();
        ZoomControls::ensure_type();
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.set_css_name("headerbar");
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for HeaderBar {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup();
    }
}

impl WidgetImpl for HeaderBar {}
impl BinImpl for HeaderBar {}

#[gtk::template_callbacks]
impl HeaderBar {
    fn setup(&self) {
        self.settings.bind_show_menu_button(&*self.menu_button, "visible").get_only().build();

        self.obj()
            .bind_property("fullscreened", &*self.header_bar, "show-end-title-buttons")
            .invert_boolean()
            .sync_create()
            .build();

        self.settings.bind_show_headerbar(&*self.revealer, "reveal-child").get_only().build();

        self.revealer.connect_child_revealed_notify(clone!(@weak self as this => move |r| {
            this.on_reveal_changed(r.is_child_revealed());
        }));

        self.obj().connect_root_notify(clone!(@weak self as this => move |obj| {
            if let Some(window) = obj.root().and_then(|root| root.clone().downcast::<gtk::Window>().ok()) {
                window.bind_property("title", obj, "title").sync_create().build();
            }
        }));

        self.set_integrated_tab_bar();

        self.settings
            .connect_headerbar_integrated_tabbar_changed(clone!(@weak self as this => move |_| {
                this.set_integrated_tab_bar();
            }));

        self.tab_bar.borrow().connect_view_notify(clone!(@weak self as this => move |tabbar| {
            this.set_integrated_tab_bar();
            if let Some(tab_view) = tabbar.view() {
                info!("tab view: {:?}", tab_view);
                tab_view.connect_n_pages_notify( move |_| {
                    this.set_integrated_tab_bar();
                });
            }
        }));
    }

    fn on_reveal_changed(&self, revealed: bool) {
        println!("on reveal changed: {:?}", revealed);
    }

    fn set_floating(&self, float: bool) {
        // if (self.is_floating.get() && float) {
        //     return;
        // }

        // this.setting_header_bar_to_floating = true;

        // if (should_float && this.header_bar_revealer.parent != this.overlay) {
        // // ...
        // yield this.wait_for_header_bar_animation ();
        // this.layout_box.remove (this.header_bar_revealer);
        // this.overlay.add_overlay (this.header_bar_revealer);
        // }
        // else if (!should_float && this.header_bar_revealer.parent != this.layout_box) {
        // // ...
        // this.overlay.remove_overlay (this.header_bar_revealer);
        // this.layout_box.prepend (this.header_bar_revealer);
        // }

        // this.setting_header_bar_to_floating = false;
    }

    fn set_integrated_tab_bar(&self) {
        let tab_bar = self.tab_bar.borrow();

        if self.settings.headerbar_integrated_tabbar() {
            if self.header_bar.title_widget() != Some(tab_bar.clone().into()) {
                tab_bar.unparent();
                self.header_bar.set_title_widget(Some(&*tab_bar));
            }
            tab_bar.set_halign(gtk::Align::Fill);
            tab_bar.set_hexpand(true);
            tab_bar.set_autohide(false);
            tab_bar.set_can_focus(false);
            tab_bar.set_css_classes(&["inline", "integrated"]);
        } else {
            self.header_bar.set_title_widget(Some(&*self.title_widget));
            self.header_box.append(&*tab_bar);

            tab_bar.set_halign(gtk::Align::Fill);
            tab_bar.set_hexpand(true);
            tab_bar.set_autohide(true);
            tab_bar.set_can_focus(false);
            tab_bar.set_css_classes(&[]);
        }
    }
}
