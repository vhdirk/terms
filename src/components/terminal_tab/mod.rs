mod imp;
use std::path::PathBuf;

use glib::closure_local;
use gtk::{prelude::*, subclass::prelude::*};

use crate::util::EnvMap;

glib::wrapper! {
        pub struct TerminalTab(ObjectSubclass<imp::TerminalTab>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TerminalTab {
    pub fn new(directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("directory", directory)
            .property("command", command)
            .property("env", env)
            .build()
    }

    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "close",
            true,
            closure_local!(move |obj: TerminalTab| {
                f(&obj);
            }),
        )
    }

    pub fn split(&self, orientation: Option<gtk::Orientation>) {
        self.imp().split(orientation);
    }
}
