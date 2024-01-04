mod style_switcher;
use glib::{closure_local, subclass::prelude::*};
use gtk::prelude::*;
use style_switcher as imp;

glib::wrapper! {
        pub struct StyleSwitcher(ObjectSubclass<imp::StyleSwitcher>)
                @extends gtk::Widget, @implements gtk::Accessible;
}

impl StyleSwitcher {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj
    }
}
