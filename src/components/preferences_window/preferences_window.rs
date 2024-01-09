use std::path::PathBuf;

use adw::prelude::{ComboRowExt, ExpanderRowExt};
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{self, clone};
use gtk::prelude::*;
use itertools::Itertools;
use once_cell::sync::Lazy;
use tracing::*;

use crate::components::ThemeThumbnail;
use crate::config::THEMES_URL;
use crate::services::settings::{ScrollbackMode, Settings, StylePreference, WorkingDirectoryMode};
use crate::services::theme_provider::ThemeProvider;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/preferences_window.ui")]
pub struct PreferencesWindow {
    pub settings: Settings,

    // Behaviour
    #[template_child]
    pub remember_window_size_switch: TemplateChild<adw::SwitchRow>,

    // Terminal - Text
    #[template_child]
    pub system_font_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub custom_font_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub custom_font_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub cell_width_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub cell_height_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub bold_is_bright_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub easy_copy_paste_switch: TemplateChild<adw::SwitchRow>,

    // Terminal - Terminal
    #[template_child]
    pub terminal_bell_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub cursor_shape_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub cursor_blink_mode_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub padding_spin_button_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub opacity_spin_button_adjustment: TemplateChild<gtk::Adjustment>,

    // Terminal - Scrolling
    #[template_child]
    pub scrollback_mode_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub custom_scrollback_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub custom_scrollback_spin_button: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub show_scrollbars_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub use_overlay_scrolling_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub scroll_on_keystroke_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub scroll_on_output_switch: TemplateChild<adw::SwitchRow>,

    // Terminal - Working directory
    #[template_child]
    pub working_directory_mode_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub custom_working_directory_entry_row: TemplateChild<adw::EntryRow>,

    // Terminal - Command
    #[template_child]
    pub run_command_as_login_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub use_custom_shell_command_switch: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub custom_command_entry_row: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub notify_process_completion_switch: TemplateChild<adw::SwitchRow>,

    // Terminal - Appearance
    #[template_child]
    pub style_preference_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub theme_integration_switch: TemplateChild<adw::SwitchRow>,

    // Terminal - Theme
    #[template_child]
    pub filter_themes_check_button: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_theme_toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub light_theme_toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub preview_flow_box: TemplateChild<gtk::FlowBox>,

    #[template_child]
    pub scrollbars_expander_row: TemplateChild<adw::ExpanderRow>,
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

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| vec![glib::ParamSpecString::builder("selected-theme").readwrite().build()]);

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "selected-theme" => {
                if let Ok(theme) = value.get::<String>() {
                    info!("Setting theme (light: {:?}): {:?}", self.light_theme_toggle.is_active(), theme);

                    if self.light_theme_toggle.is_active() {
                        self.settings.set_theme_light(&theme);
                    } else {
                        self.settings.set_theme_dark(&theme);
                    }
                }
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "selected-theme" => if self.light_theme_toggle.is_active() {
                self.settings.theme_light()
            } else {
                self.settings.theme_dark()
            }
            .to_value(),
            _ => unimplemented!(),
        }
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

        // TODO: this is very slow when dealing with lots of themes
        for (name, theme) in ThemeProvider::default().themes().iter().sorted_by_key(|x| x.0) {
            let thumb = ThemeThumbnail::new(theme);

            let theme_name_to = name.clone();
            let theme_name_from = name.clone();

            self.obj()
                .bind_property("selected-theme", &thumb, "selected")
                .sync_create()
                .bidirectional()
                .transform_to(move |_, from: String| Some(from == theme_name_to))
                .transform_from(move |_, to: bool| if to { Some(theme_name_from.clone()) } else { None })
                .build();

            self.preview_flow_box.append(&thumb);
        }

        self.bind_data();
    }

    fn connect_signals(&self) {}

    fn bind_data(&self) {
        // Behavior
        self.settings.bind_remember_window_size(&*self.remember_window_size_switch, "active").build();

        // Terminal - Text
        self.settings.bind_system_font(&*self.system_font_switch, "active").build();
        self.settings.bind_custom_font(&*self.custom_font_label, "label").build();
        self.settings.bind_system_font(&*self.custom_font_row, "visible").invert_boolean().build();
        self.settings.bind_terminal_cell_width(&*self.cell_width_spacing_adjustment, "value").build();
        self.settings.bind_terminal_cell_height(&*self.cell_height_spacing_adjustment, "value").build();
        self.settings.bind_theme_bold_is_bright(&*self.bold_is_bright_switch, "active").build();
        self.settings.bind_easy_copy_paste(&*self.easy_copy_paste_switch, "active").build();

        // Terminal - Terminal
        self.settings.bind_terminal_bell(&*self.terminal_bell_switch, "active").build();
        self.settings.bind_cursor_shape(&*self.cursor_shape_combo_row, "selected").build();
        self.settings.bind_cursor_blink_mode(&*self.cursor_blink_mode_combo_row, "selected").build();

        // TODO: why store each side when we don't have the ability to adjust them individually
        self.settings
            .bind_terminal_padding(&*self.padding_spin_button_adjustment, "value")
            .mapping(|variant, _ty| {
                variant
                    .get::<(u32, u32, u32, u32)>()
                    .map(|(top, _right, _bottom, _left)| (top as f64).to_value())
            })
            .set_mapping(|value, _ty| {
                value
                    .get::<f64>()
                    .ok()
                    .map(|pad| ((pad as u32, pad as u32, pad as u32, pad as u32).to_variant()))
            })
            .build();

        self.settings
            .bind_opacity(&*self.opacity_spin_button_adjustment, "value")
            .mapping(|variant, _ty| variant.get::<u32>().map(|value| (value as f64).to_value()))
            .set_mapping(|value, _ty| value.get::<f64>().ok().map(|value| (value as u32).to_variant()))
            .build();

        // Terminal - Scrolling
        self.settings.bind_show_scrollbars(&*self.show_scrollbars_switch, "active").build();
        self.settings.bind_use_overlay_scrolling(&*self.use_overlay_scrolling_switch, "active").build();
        self.settings
            .bind_scrollback_lines(&*self.custom_scrollback_adjustment, "value")
            .mapping(|variant, _ty| variant.get::<u32>().map(|value| (value as f64).to_value()))
            .set_mapping(|value, _ty| value.get::<f64>().ok().map(|value| (value as u32).to_variant()))
            .build();

        self.settings
            .bind_scrollback_mode(&*self.scrollback_mode_combo_row, "selected")
            .mapping(|variant, _ty| variant.get::<ScrollbackMode>().map(|mode| (mode as u32).to_value()))
            .set_mapping(|value, _ty| value.get::<u32>().ok().map(|v| ScrollbackMode::from(v).into()))
            .build();

        self.settings.connect_scrollback_mode_changed(clone!(@weak self as this => move|s| {
            this.custom_scrollback_spin_button.set_sensitive(s.scrollback_mode() == ScrollbackMode::FixedSize);
        }));
        self.custom_scrollback_spin_button
            .set_sensitive(self.settings.scrollback_mode() == ScrollbackMode::FixedSize);

        self.settings.bind_scroll_on_keystroke(&*self.scroll_on_keystroke_switch, "active").build();
        self.settings.bind_scroll_on_output(&*self.scroll_on_output_switch, "active").build();

        // Terminal - Working directory
        self.settings
            .bind_working_directory_mode(&*self.working_directory_mode_combo_row, "selected")
            .mapping(|variant, _ty| variant.get::<WorkingDirectoryMode>().map(|mode| (mode as u32).to_value()))
            .set_mapping(|value, _ty| value.get::<u32>().ok().map(|v| WorkingDirectoryMode::from(v).into()))
            .build();

        self.settings.connect_working_directory_mode_changed(clone!(@weak self as this => move|s| {
            this.set_custom_working_dir_row_visible();
        }));
        self.set_custom_working_dir_row_visible();

        self.settings
            .bind_custom_working_directory(&*self.custom_working_directory_entry_row, "text")
            .build();
        self.settings.connect_custom_working_directory_changed(clone!(@weak self as this => move|_| {
            this.validate_custom_working_directory();
        }));
        self.validate_custom_working_directory();

        // Terminal - Command
        self.settings.bind_command_as_login_shell(&*self.run_command_as_login_switch, "active").build();
        self.settings.bind_custom_shell_command(&*self.custom_command_entry_row, "text").build();
        self.settings.bind_use_custom_command(&*self.custom_command_entry_row, "sensitive").build();
        self.settings.bind_use_custom_command(&*self.use_custom_shell_command_switch, "active").build();
        self.settings
            .bind_notify_process_completion(&*self.notify_process_completion_switch, "active")
            .build();

        // Terminal - Appearance
        self.settings
            .bind_style_preference(&*self.style_preference_combo_row, "selected")
            .mapping(|variant, _ty| variant.get::<StylePreference>().map(|pref| (pref as u32).to_value()))
            .set_mapping(|value, _ty| value.get::<u32>().ok().map(|v| StylePreference::from(v).into()))
            .build();

        self.settings.bind_theme_integration(&*self.theme_integration_switch, "active").build();

        // Terminal - Theme
        self.light_theme_toggle.connect_active_notify(clone!(@weak self as this => move|_| {
            this.obj().notify("selected-theme");
            this.set_themes_filter_func();
        }));

        self.settings.connect_theme_light_changed(clone!(@weak self as this => move|_| {
            if this.light_theme_toggle.is_active() {
                this.obj().notify("selected-theme");
            }
        }));

        self.settings.connect_theme_dark_changed(clone!(@weak self as this => move|_| {
            if this.dark_theme_toggle.is_active() {
                this.obj().notify("selected-theme");
            }
        }));

        // need to use themeprovider here to listen to both settings and adw stylemanager
        ThemeProvider::default()
            .bind_property("dark", &*self.dark_theme_toggle, "active")
            .sync_create()
            .build();
        ThemeProvider::default()
            .bind_property("dark", &*self.light_theme_toggle, "active")
            .invert_boolean()
            .sync_create()
            .build();

        self.filter_themes_check_button.connect_active_notify(clone!(@weak self as this => move|_| {
            this.set_themes_filter_func();
        }));

        self.set_themes_filter_func();
    }

    #[template_callback]
    fn on_custom_font_row_activated(&self, _row: &adw::ActionRow) {
        let dialog = gtk::FontDialog::builder().title(gettext("Terminal Font")).build();

        let filter = gtk::BoolFilter::builder()
            .expression(gtk::ClosureExpression::new::<bool>(
                &[] as &[gtk::Expression],
                glib::closure!(|arg: Option<glib::Object>| {
                    arg.and_then(|a| {
                        a.downcast_ref::<pango::FontFace>()
                            .map(|face| face.family())
                            .or(a.downcast_ref::<pango::FontFamily>().cloned())
                            .map(|f| f.is_monospace())
                    })
                    .unwrap_or(false)
                }),
            ))
            .build();
        dialog.set_filter(Some(&filter));

        let font = pango::FontDescription::from_string(&self.settings.custom_font());
        // TODO: for some reason, initial font doesn't do anything
        dialog.choose_font(
            Some(&self.obj().clone()),
            Some(&font),
            None::<&gio::Cancellable>,
            clone!(@weak self as this => move |result| {
                match result {
                    Ok(font) => {
                        this.settings.set_custom_font(&font.to_str());
                    },
                    _ => ()
                }
            }),
        );
    }

    #[template_callback]
    fn set_custom_working_dir_to_home(&self, _btn: &gtk::Button) {
        self.settings.set_custom_working_directory(&dirs::home_dir().unwrap_or(PathBuf::from("/")));
    }

    fn set_custom_working_dir_row_visible(&self) {
        self.custom_working_directory_entry_row
            .set_visible(self.settings.working_directory_mode() == WorkingDirectoryMode::Custom)
    }

    fn validate_custom_working_directory(&self) {
        let path = self.settings.custom_working_directory();

        if path.exists() && path.is_dir() {
            self.custom_working_directory_entry_row.remove_css_class("error");
        } else {
            self.custom_working_directory_entry_row.add_css_class("error");
        }
    }

    fn set_themes_filter_func(&self) {
        self.preview_flow_box.set_filter_func(if !self.filter_themes_check_button.is_active() {
            info!("Showing all themes");
            |_: &gtk::FlowBoxChild| true
        } else if self.light_theme_toggle.is_active() {
            info!("Showing only light themes");
            |child: &gtk::FlowBoxChild| {
                child
                    .downcast_ref::<ThemeThumbnail>()
                    .and_then(|thumb| thumb.theme())
                    .map_or(false, |theme| !theme.is_dark())
            }
        } else {
            info!("Showing only dark themes");
            |child: &gtk::FlowBoxChild| {
                child
                    .downcast_ref::<ThemeThumbnail>()
                    .and_then(|thumb| thumb.theme())
                    .map_or(false, |theme| theme.is_dark())
            }
        });
    }

    #[template_callback]
    fn on_open_themes_folder(&self) {
        glib::spawn_future_local(
            gtk::FileLauncher::new(ThemeProvider::user_themes_dir().as_ref().map(gio::File::for_path).as_ref()).launch_future(Some(&self.obj().clone())),
        );
    }

    #[template_callback]
    fn on_get_more_themes_online(&self) {
        glib::spawn_future_local(gtk::UriLauncher::new(THEMES_URL).launch_future(Some(&self.obj().clone())));
    }
}
