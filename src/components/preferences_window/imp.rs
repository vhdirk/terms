use adw::{prelude::*, subclass::prelude::*};
use gettextrs::gettext;
use glib::clone;

use crate::components::shortcuts_preferences_page::ShortcutsPreferencesPage;
use crate::components::terminal_preferences_page::TerminalPreferencesPage;
use crate::settings::Settings;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/preferences_window.ui")]
pub struct PreferencesWindow {
    pub settings: Settings,

    // Behaviour
    #[template_child]
    pub remember_window_size_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub show_headerbar_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub headerbar_integrated_tabbar_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub show_menu_button_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub floating_controls_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub floating_controls_hover_area_adjustment: TemplateChild<gtk::Adjustment>,

    #[template_child]
    pub floating_controls_delay_adjustment: TemplateChild<gtk::Adjustment>,

    #[template_child]
    pub expand_tabs_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub show_panel_headers_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub small_panel_headers_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub wide_panel_resize_handle_switch: TemplateChild<adw::SwitchRow>,

    #[template_child]
    pub terminal_preferences_page: TemplateChild<TerminalPreferencesPage>,

    #[template_child]
    pub shortcuts_preferences_page: TemplateChild<ShortcutsPreferencesPage>,
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesWindow {
    const NAME: &'static str = "TermsPreferencesWindow";
    type Type = super::PreferencesWindow;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }
}

impl WidgetImpl for PreferencesWindow {}
impl WindowImpl for PreferencesWindow {}
impl AdwWindowImpl for PreferencesWindow {}
impl PreferencesWindowImpl for PreferencesWindow {}

#[gtk::template_callbacks]
impl PreferencesWindow {
    fn setup_widgets(&self) {
        // TODO: seems so trivial. Can't we do this in XML?
        self.terminal_preferences_page.set_window(self.obj().clone());
        self.shortcuts_preferences_page.set_window(self.obj().clone());

        self.connect_signals();

        self.bind_data();
    }

    fn connect_signals(&self) {}

    fn bind_data(&self) {
        // Behavior
        self.settings.bind_remember_window_size(&*self.remember_window_size_switch, "active").build();

        self.settings.bind_show_menu_button(&*self.show_menu_button_switch, "active").build();
        self.settings.bind_show_headerbar(&*self.show_headerbar_switch, "active").build();
        self.settings
            .bind_headerbar_integrated_tabbar(&*self.headerbar_integrated_tabbar_switch, "active")
            .build();

        self.settings.bind_floating_controls(&*self.floating_controls_switch, "active").build();
        self.settings
            .bind_floating_controls_hover_area(&*self.floating_controls_hover_area_adjustment, "value")
            .build();
        self.settings
            .bind_delay_before_showing_floating_controls(&*self.floating_controls_delay_adjustment, "value")
            .build();

        self.settings.bind_expand_tabs(&*self.expand_tabs_switch, "active").build();
        self.settings.bind_show_panel_headers(&*self.show_panel_headers_switch, "active").build();
        self.settings
            .bind_use_wide_panel_resize_handle(&*self.wide_panel_resize_handle_switch, "active")
            .build();
        self.settings.bind_small_panel_headers(&*self.small_panel_headers_switch, "active").build();
    }

    #[template_callback]
    fn on_reset_request(&self) {
        let dialog = adw::MessageDialog::builder()
            .transient_for(&*self.obj())
            .modal(true)
            .title(gettext("Reset all?"))
            .body(gettext("Are you sure you want to reset all settings to their defaults?"))
            .build();

        dialog.add_responses(&[("cancel", &gettext("_Cancel")), ("reset", &gettext("_Reset all preferences"))]);
        dialog.set_response_appearance("reset", adw::ResponseAppearance::Destructive);
        dialog.present();

        dialog.connect_response(
            None,
            clone!(@weak self as this => move |_d, resp| {
                if resp == "reset" {
                    this.settings.reset_all();
                }
            }),
        );
    }
}
