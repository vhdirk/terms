use std::{collections::HashMap, path::PathBuf};

use glib::{subclass::types::ObjectSubclassIsExt, ObjectExt};
use ref_thread_local::{ref_thread_local, RefThreadLocal};

mod theme;
pub use theme::Theme;

mod theme_provider;
use theme_provider as imp;

ref_thread_local! {
    static managed INSTANCE: ThemeProvider = ThemeProvider::new();
}

impl Default for ThemeProvider {
    fn default() -> Self {
        INSTANCE.borrow().clone()
    }
}

glib::wrapper! {
        pub struct ThemeProvider(ObjectSubclass<imp::ThemeProvider>);
}

impl ThemeProvider {
    fn new() -> Self {
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

    pub fn user_themes_dir() -> Option<PathBuf> {
        imp::ThemeProvider::user_themes_dir()
    }
}
