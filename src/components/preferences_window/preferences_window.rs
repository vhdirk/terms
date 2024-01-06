use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{self, clone};
use gtk::prelude::*;
use once_cell::sync::Lazy;
use tracing::*;

use crate::components::ThemeThumbnail;
use crate::services::settings::{Settings, StylePreference};
use crate::services::theme_provider::ThemeProvider;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/preferences_window.ui")]
pub struct PreferencesWindow {
    pub settings: Settings,
    pub theme_provider: ThemeProvider,

    // Behaviour
    #[template_child]
    pub remember_window_size_switch: TemplateChild<gtk::Switch>,

    // Terminal - Text
    #[template_child]
    pub font_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub cell_width_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub cell_height_spacing_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub bold_is_bright_switch: TemplateChild<gtk::Switch>,
    #[template_child]
    pub easy_copy_paste_switch: TemplateChild<gtk::Switch>,

    // Terminal - Terminal
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

    // Terminal - Appearance
    #[template_child]
    pub style_preference_combo_row: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub theme_integration_switch: TemplateChild<gtk::Switch>,

    // Terminal - Appearance
    #[template_child]
    pub filter_themes_check_button: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub dark_theme_toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub light_theme_toggle: TemplateChild<gtk::ToggleButton>,

    #[template_child]
    pub preview_flow_box: TemplateChild<gtk::FlowBox>,
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

        for (name, theme) in self.theme_provider.themes().iter() {
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
        self.settings.bind_font(&*self.font_label, "label").build();

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

        // settings.notify ["custom-working-directory"].connect (() => {
        //   if (this.is_custom_working_directory_valid ()) {
        //     this.custom_working_directory_entry_row.remove_css_class ("error");
        //   }
        //   else {
        //     this.custom_working_directory_entry_row.add_css_class ("error");
        //   }
        // });
        // settings.notify_property ("custom-working-directory");

        // Terminal - Appearance
        self.settings
            .bind_style_preference(&*self.style_preference_combo_row, "selected")
            .mapping(|variant, _ty| variant.get::<StylePreference>().map(|pref| (pref as u32).to_value()))
            .set_mapping(|value, _ty| value.get::<u32>().ok().map(|v| Into::<StylePreference>::into(v).into()))
            .build();

        self.settings.bind_theme_integration(&*self.theme_integration_switch, "active").build();

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

        self.theme_provider.connect_notify_local(
            Some("dark"),
            clone!(@weak self as this => move |_, _| {
                if this.theme_provider.property::<bool>("dark") {
                 this.dark_theme_toggle.set_active(true);
              }
              else {
                this.light_theme_toggle.set_active(true);
              }
            }),
        );

        self.filter_themes_check_button.connect_active_notify(clone!(@weak self as this => move|_| {
            this.set_themes_filter_func();
        }));

        self.set_themes_filter_func();
    }

    #[template_callback]
    fn on_font_row_activated(&self, _row: &adw::ActionRow) {
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

        let font = pango::FontDescription::from_string(&self.settings.font());
        // TODO: for some reason, initial font doesn't do anything
        dialog.choose_font(
            Some(&self.obj().clone()),
            Some(&font),
            None::<&gio::Cancellable>,
            clone!(@weak self as this => move |result| {
                match result {
                    Ok(font) => {
                        this.settings.set_font(&font.to_str());
                    },
                    _ => ()
                }
            }),
        );
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
        glib::spawn_future_local(gtk::UriLauncher::new("https://github.com/storm119/Tilix-Themes").launch_future(Some(&self.obj().clone())));
    }
}
