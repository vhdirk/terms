mod terminal_frame;
use std::path::PathBuf;

use glib::{closure_local, ObjectExt};
use terminal_frame as imp;

use crate::util::EnvMap;

glib::wrapper! {
        pub struct TerminalFrame(ObjectSubclass<imp::TerminalFrame>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TerminalFrame {
    pub fn new(working_directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("working-directory", working_directory)
            .property("command", command)
            .property("env", env)
            .build()
    }

    pub fn connect_exit<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "exit",
            true,
            closure_local!(move |obj: TerminalFrame| {
                f(&obj);
            }),
        )
    }
}
