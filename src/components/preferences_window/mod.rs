mod preferences_window;
use glib::subclass::prelude::*;
use preferences_window as imp;

glib::wrapper! {
        pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
                @extends gtk::Widget, gtk::Window, adw::PreferencesWindow,
                @implements gio::ActionGroup, gio::ActionMap; //, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PreferencesWindow {}
