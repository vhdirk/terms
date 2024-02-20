use glib::{closure_local, prelude::*, subclass::prelude::*};

use super::{panel_imp as imp, utils::TwlWidgetExt};

glib::wrapper! {
        pub struct Panel(ObjectSubclass<imp::Panel>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Panel {
    pub fn new(content: &impl IsA<gtk::Widget>) -> Self {
        glib::Object::builder().property("content", content).build()
    }

    pub fn set_closing(&self, closing: bool) {
        self.imp().closing.set(closing);
    }

    pub fn closing(&self) -> bool {
        self.imp().closing.get()
    }

    pub fn connect_close_request<F: Fn(&Self) -> glib::Propagation + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "close-request",
            false,
            closure_local!(move |obj: Self| { f(&obj) == glib::Propagation::Proceed }),
        )
    }
}

impl TwlWidgetExt for Panel {}
