use super::resizer_imp as imp;
use gtk::prelude::*;

glib::wrapper! {
        pub struct Resizer(ObjectSubclass<imp::Resizer>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Resizer {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
