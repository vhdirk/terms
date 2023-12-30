mod session;
use glib::{closure_local, subclass::prelude::*};
use gtk::prelude::*;
use session as imp;

use super::TerminalInitArgs;

glib::wrapper! {
        pub struct Session(ObjectSubclass<imp::Session>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
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
