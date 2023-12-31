mod window;
use glib::subclass::prelude::*;
use window as imp;

use super::{HeaderBar, Session, TerminalInitArgs};

glib::wrapper! {
        pub struct Window(ObjectSubclass<imp::Window>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window, adw::ApplicationWindow, //, panel::Workspace,
                @implements gtk::Accessible, gio::ActionGroup, gio::ActionMap; // gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P, init_args: TerminalInitArgs) -> Self {
        let this: Self = glib::Object::builder().property("application", application).build();
        this.imp().set_init_args(init_args);

        this
    }
}
