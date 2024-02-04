mod terminal_preferences_page;
use glib::prelude::*;
use terminal_preferences_page as imp;

glib::wrapper! {
        pub struct TerminalPreferencesPage(ObjectSubclass<imp::TerminalPreferencesPage>)
                @extends gtk::Widget, adw::PreferencesPage,
                @implements gio::ActionGroup, gio::ActionMap;
}

impl TerminalPreferencesPage {
    pub fn new<W: IsA<adw::PreferencesWindow>>(window: Option<&W>) -> Self {
        glib::Object::builder().property("window", window).build()
    }
}
