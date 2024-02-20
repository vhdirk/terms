/// This icon is work derived from BlackBox
/// https://gitlab.gnome.org/raggesilver/blackbox/-/blob/3264dba83b0d9a6aad28694fa8b1cc139b69d523/src/widgets/StyleSwitcher.vala
///
/// Copyright 2022 Paulo Queiroz
///
/// BlackBox is licensed GNU GPLv3
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, CompositeTemplate};
use std::cell::RefCell;
use tracing::*;

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Twl/gtk/style_switcher.ui")]
#[properties(wrapper_type=super::StyleSwitcher)]
pub struct StyleSwitcher {
    #[template_child]
    pub system_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub light_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_selector: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub layout_box: TemplateChild<gtk::Box>,

    #[property(get, set=Self::set_style_preference ,construct, default="system")]
    pub preference: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for StyleSwitcher {
    const NAME: &'static str = "TwlStyleSwitcher";
    type Type = super::StyleSwitcher;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.set_css_name("style_switcher");
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
#[glib::derived_properties]
impl ObjectImpl for StyleSwitcher {
    fn dispose(&self) {
        // for some reason we need to do this manually or gtk complains that self still has children
        self.layout_box.unparent();
    }
}

impl WidgetImpl for StyleSwitcher {}

#[gtk::template_callbacks]
impl StyleSwitcher {
    fn set_style_preference(&self, preference: &str) {
        let _system_guard = self.system_selector.freeze_notify();
        let _light_guard = self.light_selector.freeze_notify();
        let _dark_guard = self.dark_selector.freeze_notify();

        match preference {
            "system" => {
                self.system_selector.set_active(true);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(false);
            },
            "light" => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(true);
                self.dark_selector.set_active(false);
            },
            "dark" => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(true);
            },
            _ => {
                warn!("Invalid style preference: {:?}", preference)
            },
        }
    }

    #[template_callback]
    fn theme_check_active_changed(&self) {
        if self.system_selector.is_active() {
            self.preference.set("system".to_string());
        } else if self.light_selector.is_active() {
            self.preference.set("light".to_string());
        } else if self.dark_selector.is_active() {
            self.preference.set("dark".to_string());
        }
        self.obj().notify_preference();
    }
}
