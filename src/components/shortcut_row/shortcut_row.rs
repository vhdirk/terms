use crate::i18n::{gettext, gettext_f};
use crate::settings::ShortcutSettings;
use adw::prelude::PreferencesRowExt;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use tracing::*;

use std::cell::{OnceCell, RefCell};

use glib::clone;

#[derive(Default, Properties, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/shortcut_row.ui")]
#[properties(wrapper_type = super::ShortcutRow)]

pub struct ShortcutRow {
    pub settings: ShortcutSettings,

    #[property(get, construct_only)]
    pub action: OnceCell<String>,

    #[property(get, construct_only)]
    pub title: OnceCell<String>,

    #[template_child]
    pub accelerators_box: TemplateChild<gtk::Box>,

    #[template_child]
    pub popover: TemplateChild<gtk::PopoverMenu>,
}

#[glib::object_subclass]
impl ObjectSubclass for ShortcutRow {
    const NAME: &'static str = "TermsShortcutRow";
    type Type = super::ShortcutRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for ShortcutRow {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup();
    }
}

impl WidgetImpl for ShortcutRow {}
impl ListBoxRowImpl for ShortcutRow {}
impl PreferencesRowImpl for ShortcutRow {}
impl ActionRowImpl for ShortcutRow {}

#[gtk::template_callbacks]
impl ShortcutRow {
    fn setup(&self) {
        self.clear_accelerators();
        if let Some(action) = self.action.get() {
            let key = self.settings.key(action);

            self.update_accelerators(&key);

            self.settings.connect_changed(
                Some(&key),
                clone!(@weak self as this => move|_, key| {
                    this.clear_accelerators();
                    this.update_accelerators(key);
                }),
            );
        }
    }

    fn update_accelerators(&self, key: &str) {
        let accelerators = self.settings.accels(key);
        let action = self.settings.action(key);
        info!("action {:?} has accelerators {:?}", action, accelerators);

        let menu = gio::Menu::new();

        let add_shortcut = gio::MenuItem::new(Some(&gettext("Add shortcut")), None);
        add_shortcut.set_action_and_target_value(
            Some(super::ACTION_ADD_SHORTCUT),
            Some(&(self.title.get().unwrap_or(&action), action.clone()).to_variant()),
        );
        menu.append_item(&add_shortcut);

        let reset_shortcuts = gio::MenuItem::new(Some(&gettext("Reset shortcuts")), None);
        reset_shortcuts.set_action_and_target_value(Some(super::ACTION_RESET_SHORTCUTS), Some(&action.to_variant()));
        menu.append_item(&reset_shortcuts);

        self.popover.set_menu_model(Some(&menu));

        if accelerators.is_empty() {
            self.accelerators_box
                .append(&gtk::Label::builder().label(gettext("Disabled")).css_classes(["dim-label"]).build());

            return;
        }

        let section = gio::Menu::new();
        for accel in accelerators.iter() {
            let item = gio::MenuItem::new(
                Some(&gettext_f("Remove {shortcut}", &[("shortcut", &self.settings.accel_as_label(&accel))])),
                None,
            );
            item.set_action_and_target_value(Some(super::ACTION_REMOVE_SHORTCUT), Some(&accel.to_variant()));
            section.append_item(&item);

            self.accelerators_box
                .append(&gtk::ShortcutLabel::builder().accelerator(accel).halign(gtk::Align::End).build());
        }
        menu.append_section(None, &section);
    }

    fn clear_accelerators(&self) {
        while let Some(child) = self.accelerators_box.first_child() {
            self.accelerators_box.remove(&child);
        }
    }
}
