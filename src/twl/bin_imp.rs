use adw::prelude::*;
use adw::subclass::prelude::*;

use super::utils::{twl_widget_focus_move, twl_widget_grab_focus};

#[derive(Debug, Default)]
pub struct Bin {}

#[glib::object_subclass]
impl ObjectSubclass for Bin {
    const NAME: &'static str = "TwlBin";
    type Type = super::Bin;
    type ParentType = adw::Bin;
}

impl ObjectImpl for Bin {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for Bin {
    fn move_focus(&self, direction: gtk::DirectionType) {
        twl_widget_focus_move(self.obj().as_ref(), direction);
    }

    fn grab_focus(&self) -> bool {
        twl_widget_grab_focus(self.obj().as_ref())
    }
}

impl BinImpl for Bin {}

impl Bin {}
