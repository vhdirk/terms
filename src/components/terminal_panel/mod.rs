mod terminal_panel;
use std::path::PathBuf;

use glib::{closure_local, ObjectExt};
use terminal_panel as imp;

use crate::util::EnvMap;

glib::wrapper! {
        pub struct TerminalPanel(ObjectSubclass<imp::TerminalPanel>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TerminalPanel {
    pub fn new(directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("directory", directory)
            .property("command", command)
            .property("env", env)
            .build()
    }

    pub fn connect_exit<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "exit",
            true,
            closure_local!(move |obj: TerminalPanel| {
                f(&obj);
            }),
        )
    }
}
