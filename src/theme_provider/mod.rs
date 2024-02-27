use std::{collections::HashMap, path::PathBuf};

use glib::{prelude::*, subclass::types::ObjectSubclassIsExt};
use once_cell::sync::Lazy;
use ref_thread_local::{ref_thread_local, RefThreadLocal};
use tracing::*;

mod imp;
mod theme;
pub use theme::Theme;

ref_thread_local! {
    static managed INSTANCE: Lazy<ThemeProvider> = Lazy::new(ThemeProvider::new);
}

impl Default for ThemeProvider {
    fn default() -> Self {
        Lazy::force(&INSTANCE.borrow()).clone()
    }
}

glib::wrapper! {
    pub struct ThemeProvider(ObjectSubclass<imp::ThemeProvider>);
}

impl ThemeProvider {
    fn new() -> Self {
        info!("Creating theme provider");
        glib::Object::builder().build()
    }

    pub fn themes(&self) -> HashMap<String, Theme> {
        self.imp().themes()
    }

    pub fn theme(&self, name: &str) -> Option<Theme> {
        self.themes().get(name).cloned()
    }

    pub fn current_theme_name(&self) -> Option<String> {
        self.property("current-theme")
    }

    pub fn current_theme(&self) -> Option<Theme> {
        self.current_theme_name().and_then(|t| self.themes().get(&t).cloned())
    }

    pub fn user_themes_dir() -> PathBuf {
        imp::ThemeProvider::user_themes_dir()
    }
}
