mod imp;
use glib::subclass::prelude::*;

glib::wrapper! {
        pub struct SearchToolbar(ObjectSubclass<imp::SearchToolbar>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SearchToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchToolbar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj
    }
}
