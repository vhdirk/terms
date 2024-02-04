mod tile_grid;
use tile_grid as imp;

use glib::{closure_local, ObjectExt};
use gtk::subclass::prelude::*;

use super::Tile;

glib::wrapper! {
        pub struct TileGrid(ObjectSubclass<imp::TileGrid>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TileGrid {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn add(&self, child: &impl glib::IsA<gtk::Widget>, orientation: gtk::Orientation) -> Tile {
        self.imp().add(child, orientation)
    }

    pub fn close_other_tiles(&self, tile: &Tile) {
        self.imp().close_other_tiles(tile)
    }

    pub fn close_tile(&self, tile: &Tile) {
        self.imp().close_tile(tile)
    }
}
