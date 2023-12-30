mod header_bar;
use gtk::prelude::*;
use header_bar as imp;

use crate::{application::AppProfile, config::PROFILE};

glib::wrapper! {
        pub struct HeaderBar(ObjectSubclass<imp::HeaderBar>)
                @extends gtk::Widget, gtk::Window, gtk::HeaderBar,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl HeaderBar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();

        match PROFILE {
            AppProfile::Devel => {
                obj.set_css_classes(&["devel"]);
            },
            _ => (),
        }
        obj
    }
}
