use super::{panel_header_imp as imp, utils::TwlWidgetExt};

glib::wrapper! {
        pub struct PanelHeader(ObjectSubclass<imp::PanelHeader>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for PanelHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl PanelHeader {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl TwlWidgetExt for PanelHeader {}
