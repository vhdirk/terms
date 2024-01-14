mod constants;
mod spawn;
mod terminal;
use std::{collections::HashMap, path::PathBuf};

use glib::{closure_local, subclass::prelude::*, ObjectExt};
use terminal as imp;

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
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
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
