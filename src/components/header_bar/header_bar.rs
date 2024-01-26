use std::cell::{Cell, RefCell};

use adw::subclass::prelude::*;
use glib::{self, Properties, StaticTypeExt};
use glib::{clone, prelude::*};
use vte::WidgetExt;

use crate::components::StyleSwitcher;
use crate::settings::Settings;

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
    pub menu_button: TemplateChild<gtk::MenuButton>,

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
        klass.bind_template();
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
}
