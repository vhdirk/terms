mod terminal_panel;
use glib::{closure_local, subclass::prelude::*, ObjectExt};
use terminal_panel as imp;

use super::TerminalInitArgs;

glib::wrapper! {
        pub struct TerminalPanel(ObjectSubclass<imp::TerminalPanel>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TerminalPanel {
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
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
