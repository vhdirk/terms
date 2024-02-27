mod constants;
mod imp;
mod spawn;
use std::path::PathBuf;

use glib::{closure_local, prelude::*};

use crate::util::EnvMap;

glib::wrapper! {
        pub struct Terminal(ObjectSubclass<imp::Terminal>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl Terminal {
    pub fn new(directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("directory", directory)
            .property("command", command)
            .property("env", env)
            .build()
    }

    pub fn connect_exit<F: Fn(&Self, i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "exit",
            true,
            closure_local!(move |obj: Terminal, status: i32| {
                f(&obj, status);
            }),
        )
    }
}
