use super::style_switcher_imp as imp;
use glib::{closure_local, subclass::prelude::*};
use gtk::prelude::*;

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
