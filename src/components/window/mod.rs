mod window;
use std::path::PathBuf;

use glib::subclass::prelude::*;
use window as imp;

use crate::util::EnvMap;

use super::{HeaderBar, TerminalTab};

glib::wrapper! {
        pub struct Window(ObjectSubclass<imp::Window>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window, adw::ApplicationWindow, //, panel::Workspace,
                @implements gtk::Accessible, gio::ActionGroup, gio::ActionMap, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P, command: Option<String>, directory: Option<PathBuf>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("application", application)
            .property("command", command)
            .property("directory", directory)
            .property("env", env)
            .build()
    }
}
