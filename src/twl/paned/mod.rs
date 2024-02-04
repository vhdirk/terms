mod paned;
use paned as imp;

use glib::{closure_local, ObjectExt};

glib::wrapper! {
        pub struct Paned(ObjectSubclass<imp::Paned>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Paned {
    pub fn new(orientation: gtk::Orientation) -> Self {
        glib::Object::builder().property("orientation", orientation).build()
    }
}
