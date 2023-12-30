mod search_toolbar;
use glib::subclass::prelude::*;
use search_toolbar as imp;

glib::wrapper! {
        pub struct SearchToolbar(ObjectSubclass<imp::SearchToolbar>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchToolbar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj
    }
}
