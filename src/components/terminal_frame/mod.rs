mod terminal_frame;
use glib::{closure_local, subclass::prelude::*, ObjectExt};
use terminal_frame as imp;

use super::TerminalInitArgs;

glib::wrapper! {
        pub struct TerminalFrame(ObjectSubclass<imp::TerminalFrame>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TerminalFrame {
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
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
