use adw::ffi::AdwPreferencesWindow;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, closure_local, RustClosure};
use gtk::prelude::*;
use gtk::{gio, glib};
use panel::subclass::prelude::*;
use std::cell::RefCell;

use crate::services::settings::Settings;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/preferences_window.ui")]
pub struct PreferencesWindow {
    pub settings: Settings,

    // Behaviour
    #[template_child]
    pub remember_window_size_switch: TemplateChild<gtk::Switch>,

    // Terminal
    #[template_child]
    pub font_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub cell_width_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub cell_height_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub bold_is_bright_switch: TemplateChild<gtk::Switch>,

    #[template_child]
    pub terminal_bell_switch: TemplateChild<gtk::Switch>,
    #[template_child]
    pub cursor_shape_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub cursor_blink_mode_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub padding_spin_button_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub opacity_spin_button_adjustment: TemplateChild<gtk::Adjustment>,
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
        self.connect_signals();
        self.bind_data();
    }

    fn connect_signals(&self) {}

    fn bind_data(&self) {
        self.settings
            .bind_remember_window_size(&self.remember_window_size_switch.clone(), "active")
            .build();
        self.settings.bind_font(&self.font_label.clone(), "label").build();

        self.settings
            .bind_terminal_cell_width(&self.cell_width_spacing_adjustment.clone(), "value")
            .build();
        self.settings
            .bind_terminal_cell_height(&self.cell_height_spacing_adjustment.clone(), "value")
            .build();
        self.settings.bind_theme_bold_is_bright(&self.bold_is_bright_switch.clone(), "active").build();

        self.settings.bind_terminal_bell(&self.terminal_bell_switch.clone(), "active").build();
        self.settings.bind_cursor_shape(&self.cursor_shape_combo_row.clone(), "selected").build();
        self.settings
            .bind_cursor_blink_mode(&self.cursor_blink_mode_combo_row.clone(), "selected")
            .build();

        // self.settings.bind_terminal_padding(&self.cursor_blink_combo_row.clone(), "selected").build();
        self.settings.bind_opacity(&self.opacity_spin_button_adjustment.clone(), "value").build();
    }

    #[template_callback]
    fn on_font_row_activated(&self, _row: &adw::ActionRow) {
        let chooser = gtk::FontChooserDialog::builder()
            .title(gettext("Terminal Font"))
            .transient_for(&self.obj().clone())
            .level(gtk::FontChooserLevel::FAMILY | gtk::FontChooserLevel::SIZE | gtk::FontChooserLevel::STYLE)
            .font(self.settings.font())
            .build();
        chooser.set_filter_func(|family, _| family.is_monospace());

        chooser.connect_response(clone!(@weak self as this => move |chooser, response: gtk::ResponseType| {
            println!("response {:?}", response);
            if response == gtk::ResponseType::Ok && chooser.font().is_some() {
                this.settings.set_font(&chooser.font().unwrap())
            }
            chooser.destroy();
        }));

        chooser.show();
    }
}
