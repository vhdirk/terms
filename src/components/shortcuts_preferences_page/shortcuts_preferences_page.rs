use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::{self, clone, Properties};
use gtk::prelude::*;
use tracing::*;

use crate::{
    components::{ShortcutDialog, ShortcutRow, ACTION_ADD_SHORTCUT, ACTION_REMOVE_SHORTCUT, ACTION_RESET_ALL_SHORTCUTS, ACTION_RESET_SHORTCUTS},
    settings::Settings,
};

#[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/shortcuts_preferences_page.ui")]
#[properties(wrapper_type = super::ShortcutsPreferencesPage)]
pub struct ShortcutsPreferencesPage {
    pub settings: Settings,

    #[property(get, set)]
    pub window: RefCell<Option<adw::PreferencesWindow>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ShortcutsPreferencesPage {
    const NAME: &'static str = "TermsShortcutsPreferencesPage";
    type Type = super::ShortcutsPreferencesPage;
    type ParentType = adw::PreferencesPage;

    fn class_init(klass: &mut Self::Class) {
        ShortcutRow::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();

        klass.install_action(ACTION_ADD_SHORTCUT, Some("(ss)"), |page: &super::ShortcutsPreferencesPage, _, payload| {
            if let Some((title, action_name)) = payload.and_then(|v| v.get::<(String, String)>()) {
                page.imp().add_shortcut(&title, &action_name);
            }
        });

        klass.install_action(ACTION_REMOVE_SHORTCUT, Some("s"), |page: &super::ShortcutsPreferencesPage, _, payload| {
            if let Some(shortcut) = payload.and_then(|v| v.get::<String>()) {
                page.imp().remove_shortcut(&shortcut);
            }
        });

        klass.install_action(ACTION_RESET_SHORTCUTS, Some("s"), |page: &super::ShortcutsPreferencesPage, _, payload| {
            if let Some(action_name) = payload.and_then(|v| v.get::<String>()) {
                page.imp().reset_shortcuts(&action_name);
            }
        });

        klass.install_action(ACTION_RESET_ALL_SHORTCUTS, None, |page: &super::ShortcutsPreferencesPage, _, payload| {
            page.imp().reset_all_shortcuts();
        });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for ShortcutsPreferencesPage {}

impl WidgetImpl for ShortcutsPreferencesPage {}
impl PreferencesPageImpl for ShortcutsPreferencesPage {}

#[gtk::template_callbacks]
impl ShortcutsPreferencesPage {
    fn add_shortcut(&self, title: &str, action: &str) {
        info!("Request to add shortcut for {:?} ({:?})", title, action);
        let shortcut_settings = self.settings.shortcuts();

        let key = shortcut_settings.key(action);
        let dialog = ShortcutDialog::new(self.obj().window().as_ref(), title);
        dialog.connect_response(clone!(@weak self as this => move |_, response, accel| {
            info!("got response from dialog: {:?}, {:?}", response, accel);
            if response != gtk::ResponseType::Apply {
                return;
            }

            if let Some(accel) = accel {
                shortcut_settings.add_accel(&key, &accel);
            }
        }));

        dialog.present();
    }

    fn remove_shortcut(&self, accel: &str) {
        info!("Request to remove accel {:?}", accel);
        let shortcut_settings = self.settings.shortcuts();
        shortcut_settings.remove_accel(accel);
    }

    fn reset_shortcuts(&self, action: &str) {
        info!("Request to reset shortcuts for {:?}", action);
        let shortcut_settings = self.settings.shortcuts();

        let key = shortcut_settings.key(action);
        shortcut_settings.reset(&key);
    }

    fn reset_all_shortcuts(&self) {
        info!("Request to reset all shortcuts");
        let shortcut_settings = self.settings.shortcuts();

        // TODO: confirmation needed?
        shortcut_settings.reset_all();
    }
}