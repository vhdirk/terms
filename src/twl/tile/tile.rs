use std::cell::{Cell, RefCell};

use adw::subclass::prelude::*;
use glib::prelude::*;
use glib::Properties;
use vte::BoxExt;
use vte::WidgetExt;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::Tile)]
pub struct Tile {
    container: gtk::Box,

    #[property(get, set, construct_only, nullable)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct)]
    needs_attention: Cell<bool>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<gio::Icon>>,

    #[property(get, set, construct_only, nullable)]
    tooltip: RefCell<Option<String>>,

    #[property(get, set=Self::set_child, construct, nullable)]
    child: RefCell<Option<gtk::Widget>>,
    // #[property(get, set, construct)]
    // selected: Cell<bool>,

    // #[property(get, set, construct)]
    // live_thumbnail: Cell<bool>,

    //   PAGE_PROP_PARENT,
    //   PAGE_PROP_SELECTED,
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
    const NAME: &'static str = "TwlTile";
    type Type = super::Tile;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for Tile {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup();
    }
}
impl WidgetImpl for Tile {}
impl BinImpl for Tile {}

impl Tile {
    fn setup(&self) {
        self.container.set_parent(&*self.obj());
    }

    fn set_child(&self, child: Option<gtk::Widget>) {
        *self.child.borrow_mut() = child.clone();

        if let Some(child) = child.as_ref() {
            self.container.append(child);
        }
    }
}
