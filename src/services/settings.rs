use crate::config::APP_ID;
use constcat::concat;
use gsettings_macro::gen_settings;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};
use vte::CursorBlinkMode;

#[derive(Debug, Default, Clone)]
#[repr(u8)]
pub enum TerminalApplicationStyle {
    #[default]
    System,
    Light,
    Dark,
}

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

// #[gen_settings(
//     file = "data/io.github.vhdirk.Terms.gschema.xml.in",

// )]
// pub struct SearchSettings;

// impl Default for SearchSettings {
//     fn default() -> Self {
//         Self::new(concat!(APP_ID, ".search"));
//     }
// }

// impl SearchSettings {

// }
