mod imp;
use glib::prelude::*;

glib::wrapper! {
        pub struct TerminalPreferencesPage(ObjectSubclass<imp::TerminalPreferencesPage>)
                @extends gtk::Widget, adw::PreferencesPage,
                @implements gio::ActionGroup, gio::ActionMap;
}

impl TerminalPreferencesPage {
    pub fn new(window: Option<&impl IsA<adw::PreferencesWindow>>) -> Self {
        glib::Object::builder().property("window", window).build()
    }
}
