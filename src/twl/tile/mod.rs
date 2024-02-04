mod tile;
use tile as imp;

use glib::{closure_local, prelude::*};

glib::wrapper! {
        pub struct Tile(ObjectSubclass<imp::Tile>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Tile {
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        glib::Object::builder().property("child", child).build()
    }
}
