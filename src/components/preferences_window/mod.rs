mod preferences_window;

use glib::IsA;
use preferences_window as imp;

glib::wrapper! {
        pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
                @extends gtk::Widget, gtk::Window, adw::PreferencesWindow,
                @implements gio::ActionGroup, gio::ActionMap;
}

impl PreferencesWindow {
    pub fn new<W: IsA<gtk::Window>>(parent_window: Option<&W>) -> Self {
        glib::Object::builder().property("transient-for", parent_window).build()
    }
}
