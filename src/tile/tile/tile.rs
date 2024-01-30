use std::cell::{Cell, RefCell};

use adw::subclass::prelude::*;
use glib::prelude::*;
use glib::Properties;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::Tile)]
pub struct Tile {
    #[property(get, set, construct)]
    selected: Cell<bool>,

    #[property(get, set, construct_only, nullable)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct)]
    needs_attention: Cell<bool>,
    // #[property(get, set, construct)]
    // live_thumbnail: Cell<bool>,

    //   PAGE_PROP_PARENT,
    //   PAGE_PROP_SELECTED,
    //   PAGE_PROP_TOOLTIP,
    //   PAGE_PROP_ICON,
    //   PAGE_PROP_LOADING,
    //   PAGE_PROP_INDICATOR_ICON,
    //   PAGE_PROP_INDICATOR_TOOLTIP,
    //   PAGE_PROP_INDICATOR_ACTIVATABLE,
    //   PAGE_PROP_KEYWORD,
    //   PAGE_PROP_THUMBNAIL_XALIGN,
    //   PAGE_PROP_THUMBNAIL_YALIGN,
    //   PAGE_PROP_LIVE_THUMBNAIL,
}

#[glib::object_subclass]
impl ObjectSubclass for Tile {
    const NAME: &'static str = "TileTile";
    type Type = super::Tile;
    type ParentType = adw::Bin;
}

#[glib::derived_properties]
impl ObjectImpl for Tile {}
impl WidgetImpl for Tile {}
impl BinImpl for Tile {}

impl Tile {}
