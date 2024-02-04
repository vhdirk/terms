use super::paned_imp as imp;

// A variant of gtk::Paned that supports an arbitrary number of widgets

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
