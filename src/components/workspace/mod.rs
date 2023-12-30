mod workspace;
use glib::subclass::prelude::*;
use workspace as imp;

use super::TerminalInitArgs;

glib::wrapper! {
        pub struct Workspace(ObjectSubclass<imp::Workspace>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow, panel::Workspace,
                @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Workspace {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P, init_args: TerminalInitArgs) -> Self {
        let this: Self = glib::Object::builder().property("application", application).build();
        this.imp().set_init_args(init_args);

        this
    }
}
