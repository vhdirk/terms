mod tile;
use tile as imp;

use glib::{closure_local, ObjectExt};

glib::wrapper! {
        pub struct Tile(ObjectSubclass<imp::Tile>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Tile {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
