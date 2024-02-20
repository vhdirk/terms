use glib::{prelude::*, subclass::prelude::*};

use super::{pack_box_imp as imp, utils::TwlWidgetExt};

glib::wrapper! {
        pub struct PackBox(ObjectSubclass<imp::PackBox>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PackBox {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        self.imp().append(child);
    }

    pub fn pack_start(&self, child: &impl IsA<gtk::Widget>) {
        self.imp().pack_start(child);
    }

    pub fn pack_end(&self, child: &impl IsA<gtk::Widget>) {
        self.imp().pack_end(child);
    }
}
impl TwlWidgetExt for PackBox {}
