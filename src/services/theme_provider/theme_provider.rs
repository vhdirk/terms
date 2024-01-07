use glib::{clone, subclass::prelude::*, ObjectExt, ParamSpecBuilderExt, ToValue};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::*;

use crate::{
    config::PKGDATADIR,
    services::settings::{Settings, StylePreference},
};
use once_cell::sync::Lazy;

use super::theme::Theme;

#[derive(Debug)]
#[repr(u8)]
#[allow(unused)]
pub enum ThemePaletteColorIndex {
    Background = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Purple = 5,
    Cyan = 6,
    Foreground = 7,
    LightBackground = 8,
    LightRed = 9,
    LightGreen = 10,
    LightYellow = 11,
    LightBlue = 12,
    LightPurple = 13,
    LightCyan = 14,
    LightForeground = 15,
}

#[derive(Debug)]
pub struct ThemeProviderContext {
    css_provider: Option<gtk::CssProvider>,
    themes: HashMap<String, Theme>,
}

impl Default for ThemeProviderContext {
    fn default() -> Self {
        Self {
            css_provider: Default::default(),
            themes: Default::default(),
        }
    }
}

pub struct ThemeProvider {
    settings: Settings,
    style_manager: adw::StyleManager,
    ctx: RefCell<ThemeProviderContext>,
}

impl Default for ThemeProvider {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            style_manager: adw::StyleManager::default(),
            ctx: RefCell::new(ThemeProviderContext::default()),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeProvider {
    const NAME: &'static str = "TermsThemeProvider";
    type Type = super::ThemeProvider;
    type ParentType = glib::Object;
}

#[glib::derived_properties]
impl ObjectImpl for ThemeProvider {
    fn constructed(&self) {
        self.parent_constructed();

        self.init();
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecBoolean::builder("dark").read_only().build(),
                glib::ParamSpecString::builder("current-theme").read_only().build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "dark" => self.is_dark().into(),
            "current-theme" => self.current_theme().into(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }
}

impl ThemeProvider {
    pub fn user_themes_dir() -> Option<PathBuf> {
        dirs::data_local_dir().map(|d| d.join("terms").join("schemes"))
    }
    pub fn app_themes_dir() -> PathBuf {
        PathBuf::from(PKGDATADIR).join("schemes")
    }

    fn ensure_user_themes_dir_exists() {
        if let Some(themes_dir) = Self::user_themes_dir() {
            if let Err(err) = fs::create_dir_all(&themes_dir) {
                error!("Error creating directory: {}", err);
            }
        }
    }

    fn read_color_theme(file_path: &Path) -> Option<Theme> {
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                error!("Error while reading color theme file {}: {}", file_path.to_string_lossy(), err);
                return None;
            },
        };

        match serde_json::from_str(&content) {
            Ok(value) => Some(value),
            Err(err) => {
                error!("Error while reading color theme file {}: {}", file_path.to_string_lossy(), err);
                return None;
            },
        }
    }

    fn load_color_themes(themes_dir: &Path) -> Vec<Theme> {
        if !themes_dir.exists() {
            return vec![];
        }

        match fs::read_dir(themes_dir).map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file() && entry.path().extension() == Some("json".as_ref()))
                .filter_map(|entry| Self::read_color_theme(&entry.path()))
                .collect()
        }) {
            Ok(themes) => themes,
            Err(err) => {
                error!("Error reading directory: {}", err);
                vec![]
            },
        }
    }

    fn load_all_color_themes() -> HashMap<String, Theme> {
        let mut themes = vec![];

        if let Some(themes_dir) = Self::user_themes_dir() {
            themes.append(&mut Self::load_color_themes(&themes_dir));
        }
        themes.append(&mut Self::load_color_themes(&Self::app_themes_dir()));

        themes.into_iter().fold(HashMap::new(), |mut acc, theme| {
            acc.insert(theme.name.clone(), theme);
            acc
        })
    }

    fn init(&self) {
        Self::ensure_user_themes_dir_exists();
        self.ctx.borrow_mut().themes = Self::load_all_color_themes();

        self.settings.connect_theme_integration_changed(clone!(@weak self as this => move |_| {
            this.apply_theming();
        }));

        self.settings.connect_theme_light_changed(clone!(@weak self as this => move |_| {
            this.apply_theming();
            if !this.style_manager.is_dark() {
                this.obj().notify("current-theme");
            }
        }));

        self.settings.connect_theme_dark_changed(clone!(@weak self as this => move |_| {
            this.apply_theming();
            if this.style_manager.is_dark() {
                this.obj().notify("current-theme");
            }
        }));

        // React to style-preference changes
        self.settings.connect_style_preference_changed(clone!(@weak self as this => move |_| {
            this.style_manager.set_color_scheme(this.settings.style_preference().into());
            this.apply_theming();
        }));

        self.style_manager.connect_dark_notify(clone!(@weak self as this  => move |_sm| {
            this.obj().notify("dark");
            this.obj().notify("current-theme");
        }));

        self.apply_theming();
    }

    pub fn current_theme(&self) -> String {
        if self.is_dark() {
            self.settings.theme_dark()
        } else {
            self.settings.theme_light()
        }
    }

    // If the current style is dark and a light theme is loaded, all window text
    // and icons will be illegible. Same goes for light style with dark theme
    // selected. In those cases, we need to disable theme integration.
    fn is_safe_to_enable_theme_integration(&self, theme: &Theme) -> bool {
        self.is_dark() == theme.is_dark()
    }

    fn is_dark(&self) -> bool {
        self.style_manager.is_dark()
    }

    fn apply_theming(&self) {
        let themes = self.ctx.borrow().themes.clone();
        let theme = themes.get(&self.current_theme());
        info!("Request to apply theme: {:?}", theme);

        if theme.is_none() {
            return;
        }

        let provider = if self.settings.theme_integration() && !self.is_safe_to_enable_theme_integration(theme.unwrap()) {
            info!("It is not safe to enable theme integration for this color scheme");
            None
        } else if self.settings.theme_integration() {
            info!("Applying theme: {:?}", theme);
            let provider = gtk::CssProvider::new();
            provider.load_from_bytes(&self.generate_gtk_theme(theme.unwrap()).as_bytes().into());
            Some(provider)
        } else {
            None
        };

        if let Some(display) = gdk::Display::default() {
            if let Some(old_provider) = &self.ctx.borrow_mut().css_provider.take() {
                gtk::style_context_remove_provider_for_display(&display, old_provider)
            }

            if let Some(provider) = provider.as_ref() {
                // higher priority
                gtk::style_context_add_provider_for_display(&display, provider, gtk::STYLE_PROVIDER_PRIORITY_USER + 200);
            }
            self.ctx.borrow_mut().css_provider = provider;
        }

        let _guard = self.style_manager.freeze_notify();
        self.style_manager.set_color_scheme(self.settings.style_preference().into());
    }

    pub fn themes(&self) -> HashMap<String, Theme> {
        self.ctx.borrow().themes.clone()
    }

    /// generate_gtk_theme
    ///
    /// Copyright 2021 Christian Hergert <chergert@redhat.com>
    /// Copyright 2022 Paulo Queiroz
    /// Copyright 2023 Dirk Van Haerenborgh <vhdirk@gmail.com>
    ///
    /// The following function is work derived from GNOME Text Editor, which is
    /// also licensed under the GNU General Public License version 3.
    ///
    /// Additionally, sourced from:
    /// https:gitlab.gnome.org/GNOME/gnome-text-editor/-/blob/86ceeeda2c35c3b504cfdf817d8119bd41782587/src/editor-recoloring.c
    ///
    fn generate_gtk_theme(&self, theme: &Theme) -> String {
        let mut gtk_theme = format!(
            r#"
                @define-color window_bg_color         {background_color};
                @define-color window_fg_color         {foreground_color};

                @define-color card_fg_color           @window_fg_color;
                @define-color headerbar_fg_color      @window_fg_color;
                @define-color headerbar_border_color  @window_fg_color;
                @define-color popover_fg_color        @window_fg_color;
                @define-color dialog_fg_color         @window_fg_color;
                @define-color dark_fill_bg_color      @headerbar_bg_color;
                @define-color view_bg_color           @card_bg_color;
                @define-color view_fg_color           @window_fg_color;

                /* @define-color borders                 mix(@window_fg_color, @window_bg_color, 0.8); */
            "#,
            background_color = theme.background_color.unwrap_or(gdk::RGBA::new(0.0, 0.0, 0.0, 255.0)),
            foreground_color = theme.foreground_color.unwrap_or(gdk::RGBA::new(255.0, 255.0, 255.0, 255.0)),
        );

        // Libadwaita sets border colors to foreground color at 15% opacity. This
        // works beautifuly for all background colors, but it breaks the borders we
        // draw

        if let Some(palette) = theme.palette {
            if self.style_manager.is_dark() {
                gtk_theme.push_str(&format!(
                    r#"
                        @define-color headerbar_bg_color    darker(@window_bg_color);
                        @define-color popover_bg_color      mix(@window_bg_color, white, 0.07);
                        @define-color dialog_bg_color       mix(@window_bg_color, white, 0.07);
                        @define-color card_bg_color         alpha(white, .08);
                        @define-color view_bg_color         darker(@window_bg_color);

                        @define-color accent_color            {accent_color};
                        @define-color accent_bg_color         {accent_color};
                        @define-color accent_fg_color         white;
                        @define-color destructive_color       {destructive_color};
                        @define-color success_color           {success_color};
                        @define-color warning_color           {warning_color};

                        @define-color root_context_color    mix(@window_bg_color, {destructive_color}, 0.4);
                        @define-color ssh_context_color     mix(@window_bg_color, {ssh_context_color}, 0.6);
                    "#,
                    destructive_color = palette.get(ThemePaletteColorIndex::LightRed as usize).unwrap().to_string(),
                    success_color = palette.get(ThemePaletteColorIndex::LightGreen as usize).unwrap().to_string(),
                    accent_color = palette.get(ThemePaletteColorIndex::LightBlue as usize).unwrap().to_string(),
                    warning_color = palette.get(ThemePaletteColorIndex::LightYellow as usize).unwrap().to_string(),
                    ssh_context_color = palette.get(ThemePaletteColorIndex::LightPurple as usize).unwrap().to_string()
                ));
            } else {
                gtk_theme.push_str(&format!(
                    r#"
                        @define-color headerbar_bg_color    mix(@window_bg_color, @window_fg_color, .1);
                        @define-color popover_bg_color      mix(@window_bg_color, white, .1);
                        @define-color dialog_bg_color       @window_bg_color;
                        @define-color card_bg_color         alpha(white, .6);

                        @define-color accent_color            {accent_color};
                        @define-color accent_bg_color         {accent_color};
                        @define-color accent_fg_color         white;
                        @define-color destructive_color       {destructive_color};
                        @define-color success_color           {success_color};
                        @define-color warning_color           {warning_color};

                        @define-color root_context_color    mix(@window_bg_color, {destructive_color}, 0.4);
                        @define-color ssh_context_color     mix(@window_bg_color, {ssh_context_color}, 0.6);
                    "#,
                    destructive_color = palette.get(ThemePaletteColorIndex::Red as usize).unwrap().to_string(),
                    success_color = palette.get(ThemePaletteColorIndex::Green as usize).unwrap().to_string(),
                    accent_color = palette.get(ThemePaletteColorIndex::Blue as usize).unwrap().to_string(),
                    warning_color = palette.get(ThemePaletteColorIndex::Yellow as usize).unwrap().to_string(),
                    ssh_context_color = palette.get(ThemePaletteColorIndex::Purple as usize).unwrap().to_string()
                ));
            }
        }

        gtk_theme.push_str(
            r#"
                @define-color error_color             @destructive_color;
                @define-color destructive_bg_color    @destructive_color;
                @define-color success_bg_color        @success_color;
                @define-color warning_bg_color        @warning_color;
            "#,
        );

        gtk_theme
    }
}
