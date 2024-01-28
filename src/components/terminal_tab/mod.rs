mod terminal_tab;
use std::path::PathBuf;

use glib::closure_local;
use gtk::prelude::*;
use terminal_tab as imp;

use crate::util::EnvMap;

glib::wrapper! {
        pub struct TerminalTab(ObjectSubclass<imp::TerminalTab>)
                @extends gtk::Widget, adw::Bin,
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
}
