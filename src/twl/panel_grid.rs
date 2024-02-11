use glib::prelude::*;
use gtk::subclass::prelude::*;

use super::{panel_grid_imp as imp, Panel};

glib::wrapper! {
        pub struct PanelGrid(ObjectSubclass<imp::PanelGrid>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PanelGrid {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_child(&self, child: &impl IsA<gtk::Widget>) -> Panel {
        self.imp().set_child(child)
    }

    pub fn split(&self, child: &impl IsA<gtk::Widget>, orientation: Option<gtk::Orientation>) -> Panel {
        self.imp().split(child, orientation)
    }

    pub fn close_other_panels(&self, panel: &Panel) {
        self.imp().close_other_panels(panel)
    }

    pub fn close_panel(&self, panel: &Panel) {
        self.imp().close_panel(panel)
    }

    pub fn close_panel_finish(&self, panel: &Panel) {
        self.imp().close_panel_finish(panel);
    }
}
