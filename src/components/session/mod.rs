mod session;
use std::path::PathBuf;

use glib::{closure_local, subclass::prelude::*};
use gtk::prelude::*;
use session as imp;

use crate::util::EnvMap;

use super::TerminalInitArgs;

glib::wrapper! {
        pub struct Session(ObjectSubclass<imp::Session>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(working_directory: Option<PathBuf>, command: Option<String>, env: Option<EnvMap>) -> Self {
        glib::Object::builder()
            .property("working-directory", working_directory)
            .property("command", command)
            .property("env", env)
            .build()
    }

    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "close",
            true,
            closure_local!(move |obj: Session| {
                f(&obj);
            }),
        )
    }
}
