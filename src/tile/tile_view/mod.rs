mod tile_view;
use tile_view as imp;

use glib::{closure_local, ObjectExt};

glib::wrapper! {
        pub struct TileView(ObjectSubclass<imp::TileView>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TileView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
