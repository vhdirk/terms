use std::cell::RefCell;

use adw::{prelude::BinExt, subclass::prelude::*};
use glib::Properties;
use gtk::prelude::*;

use crate::twl::Tile;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::TileGrid)]
pub struct TileGrid {
    pub bin: adw::Bin,

    #[property(get, nullable)]
    pub selected_tile: RefCell<Option<Tile>>,

    // TODO: perhaps we want to keep track of the location in the grid?
    pub tiles: RefCell<Vec<Tile>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TileGrid {
    const NAME: &'static str = "TileTileGrid";
    type Type = super::TileGrid;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for TileGrid {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }
}
impl WidgetImpl for TileGrid {}
impl BinImpl for TileGrid {}

impl TileGrid {
    pub fn setup_widgets(&self) {
        self.bin.set_parent(&*self.obj());
    }

    pub fn add(&self, child: &impl glib::IsA<gtk::Widget>, orientation: gtk::Orientation) -> Tile {
        let tile = Tile::new(child);

        if self.tiles.borrow().is_empty() {
            self.bin.set_child(Some(&tile));
        }

        self.tiles.borrow_mut().push(tile.clone());
        tile
    }

    pub fn close_other_tiles(&self, tile: &Tile) {
        todo!();
    }

    pub fn close_tile(&self, tile: &Tile) {
        todo!();
    }
}
