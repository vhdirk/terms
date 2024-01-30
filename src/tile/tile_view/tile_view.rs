use adw::subclass::prelude::*;

#[derive(Debug, Default)]
pub struct TileView {}

#[glib::object_subclass]
impl ObjectSubclass for TileView {
    const NAME: &'static str = "TileTileView";
    type Type = super::TileView;
    type ParentType = adw::Bin;
}

impl ObjectImpl for TileView {}
impl WidgetImpl for TileView {}
impl BinImpl for TileView {}

impl TileView {}
