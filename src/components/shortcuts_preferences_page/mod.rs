mod imp;
use glib::prelude::*;

glib::wrapper! {
        pub struct ShortcutsPreferencesPage(ObjectSubclass<imp::ShortcutsPreferencesPage>)
                @extends gtk::Widget, adw::PreferencesPage,
                @implements gio::ActionGroup, gio::ActionMap;
}

impl ShortcutsPreferencesPage {
    pub fn new(window: Option<&impl IsA<adw::PreferencesWindow>>) -> Self {
        glib::Object::builder().property("window", window).build()
    }
}
