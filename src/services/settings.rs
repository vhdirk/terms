use crate::config::APP_ID;
use gsettings_macro::gen_settings;
use std::path::{Path, PathBuf};

#[gen_settings(file = "data/io.github.vhdirk.Terms.gschema.xml.in")]
#[gen_settings_define(key_name = "custom-working-directory", arg_type = "&Path", ret_type = "PathBuf")]
#[gen_settings_define(signature = "(uuuu)", arg_type = "(u32,u32,u32,u32)", ret_type = "(u32,u32,u32,u32)")]
pub struct Settings;

impl Default for Settings {
    fn default() -> Self {
        Self::new(APP_ID)
    }
}

impl Settings {}

impl Into<adw::ColorScheme> for StylePreference {
    fn into(self) -> adw::ColorScheme {
        match self {
            StylePreference::System => adw::ColorScheme::Default,
            StylePreference::Light => adw::ColorScheme::ForceLight,
            StylePreference::Dark => adw::ColorScheme::ForceDark,
        }
    }
}

impl From<adw::ColorScheme> for StylePreference {
    fn from(value: adw::ColorScheme) -> Self {
        match value {
            adw::ColorScheme::ForceLight | adw::ColorScheme::PreferLight => StylePreference::Light,
            adw::ColorScheme::ForceDark | adw::ColorScheme::PreferDark => StylePreference::Dark,
            _ => StylePreference::System,
        }
    }
}

impl From<u32> for StylePreference {
    fn from(value: u32) -> Self {
        match value {
            1 => StylePreference::Light,
            2 => StylePreference::Dark,
            _ => StylePreference::System,
        }
    }
}

impl From<u32> for ScrollbackMode {
    fn from(value: u32) -> Self {
        match value {
            1 => ScrollbackMode::Unlimited,
            2 => ScrollbackMode::Disabled,
            _ => ScrollbackMode::FixedSize,
        }
    }
}

impl From<u32> for WorkingDirectoryMode {
    fn from(value: u32) -> Self {
        match value {
            1 => WorkingDirectoryMode::Home,
            2 => WorkingDirectoryMode::Custom,
            _ => WorkingDirectoryMode::PreviousTerminal,
        }
    }
}

impl Settings {
    pub fn shell_command(&self) -> Option<String> {
        if self.use_custom_command() && !self.custom_shell_command().is_empty() {
            Some(self.custom_shell_command())
        } else {
            None
        }
    }
}
