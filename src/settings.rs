use crate::{config::APP_ID, error::TermsError};
use gettextrs::gettext;
use gio::prelude::{SettingsExt, SettingsExtManual};
use glib::prelude::*;
use gsettings_macro::gen_settings;
use gtk::Settings as SystemSettings;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::info;
use vte::ShortcutTriggerExt;

#[gen_settings(file = "data/io.github.vhdirk.Terms.gschema.xml.in", id = "@app-id@", default = false)]
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

    pub fn reset_all(&self) {
        if let Some(schema) = self.settings_schema() {
            for key in schema.list_keys().iter() {
                self.reset(key);
            }
        }

        self.shortcuts().reset_all();
    }

    pub fn shortcuts(&self) -> ShortcutSettings {
        ShortcutSettings::default()
    }

    pub fn system_settings(&self) -> SystemSettings {
        SystemSettings::default().unwrap()
    }
}

#[gen_settings(file = "data/io.github.vhdirk.Terms.gschema.xml.in", id = "@app-id@.shortcuts", default = false)]
pub struct ShortcutSettings;

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self::new(&format!("{}.shortcuts", APP_ID))
    }
}

impl ShortcutSettings {
    pub fn reset_all(&self) {
        if let Some(schema) = self.settings_schema() {
            for key in schema.list_keys().iter() {
                self.reset(key);
            }
        }
    }

    pub fn action(&self, key: &str) -> String {
        key.replacen("-", ".", 1)
    }

    pub fn key(&self, action: &str) -> String {
        action.replacen(".", "-", 1)
    }

    pub fn keys(&self) -> Vec<String> {
        if let Some(schema) = self.settings_schema() {
            schema.list_keys().iter().map(ToString::to_string).collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    pub fn actions(&self) -> Vec<String> {
        self.keys().into_iter().map(|key| self.action(&key)).collect::<Vec<_>>()
    }

    pub fn accels(&self, key: &str) -> Vec<String> {
        SettingsExtManual::get::<Vec<String>>(&self.0, key)
    }

    pub fn actionmap(&self) -> HashMap<String, Vec<String>> {
        self.keys().into_iter().map(|k| ((self.action(&k), self.accels(&k)))).collect()
    }

    pub fn accel_in_use(&self, accel: &str) -> Option<String> {
        let accel = accel.to_string();
        self.keys().into_iter().find(|key| self.accels(&key).contains(&accel))
    }

    pub fn add_accel(&self, key: &str, accel: &str) {
        info!("Add accel {accel} for key {key}");
        let mut accels = self.accels(key);
        accels.push(accel.to_string());
        let _ = self.set(key, &accels);
    }

    pub fn remove_accel(&self, accel: &str) {
        let accel = accel.to_string();
        for key in self.keys() {
            let accels = self.accels(&key);
            if accels.contains(&accel) {
                let remaining = accels.into_iter().filter(|a| a != &accel).collect::<Vec<_>>();
                let _ = self.set(&key, remaining);
            }
        }
    }

    pub fn accel_as_label(&self, accel: &str) -> String {
        if let Some((keyval, modifiers)) = gtk::accelerator_parse(accel) {
            let kt = gtk::KeyvalTrigger::new(keyval, modifiers);

            if let Some(display) = gdk::Display::default() {
                kt.to_label(&display).to_string()
            } else {
                gettext("unable to display shortcut")
            }
        } else {
            gettext("invalid shortcut")
        }
    }
}
