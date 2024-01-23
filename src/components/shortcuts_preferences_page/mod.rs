mod shortcuts_preferences_page;
use glib::IsA;
use shortcuts_preferences_page as imp;

glib::wrapper! {
        pub struct ShortcutsPreferencesPage(ObjectSubclass<imp::ShortcutsPreferencesPage>)
                @extends gtk::Widget, adw::PreferencesPage,
                @implements gio::ActionGroup, gio::ActionMap;
}

impl ShortcutsPreferencesPage {
    pub fn new<W: IsA<adw::PreferencesWindow>>(window: Option<&W>) -> Self {
        glib::Object::builder().property("window", window).build()
    }
}
