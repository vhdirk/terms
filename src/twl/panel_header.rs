use std::path::PathBuf;

use super::{panel_header_imp as imp, utils::TwlWidgetExt, Panel};
use glib::closure_local;
use gtk::prelude::*;

glib::wrapper! {
        pub struct PanelHeader(ObjectSubclass<imp::PanelHeader>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PanelHeader {
    pub fn new(panel: &impl IsA<Panel>) -> Self {
        glib::Object::builder().property("panel", panel).build()
    }
}

impl TwlWidgetExt for PanelHeader {}
