use glib::prelude::*;

use super::{bin_imp as imp, utils::TwlWidgetExt};

glib::wrapper! {
        pub struct Bin(ObjectSubclass<imp::Bin>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Bin {
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        glib::Object::builder().property("child", child).build()
    }
}

impl Default for Bin {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
impl TwlWidgetExt for Bin {}
