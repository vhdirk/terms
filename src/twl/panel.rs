use glib::{prelude::*, subclass::prelude::*};

use super::panel_imp as imp;

glib::wrapper! {
        pub struct Panel(ObjectSubclass<imp::Panel>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Panel {
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        glib::Object::builder().property("child", child).build()
    }

    pub fn set_closing(&self, closing: bool) {
        self.imp().closing.set(closing);
    }

    pub fn closing(&self) -> bool {
        self.imp().closing.get()
    }
}
