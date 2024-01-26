mod constants;
mod spawn;
mod terminal;
use std::{collections::HashMap, path::PathBuf};

use glib::{closure_local, ObjectExt};
use terminal as imp;

use crate::util::EnvMap;

#[derive(Debug, Default, Clone)]
pub struct TerminalInitArgs {
    pub working_dir: Option<PathBuf>,
    pub command: Option<String>,
    pub env: HashMap<String, String>,
}

glib::wrapper! {
        pub struct Terminal(ObjectSubclass<imp::Terminal>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl Terminal {
    pub fn new(working_directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("working-directory", working_directory)
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
