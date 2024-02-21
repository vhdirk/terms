use super::zoom_controls_imp as imp;

glib::wrapper! {
        pub struct ZoomControls(ObjectSubclass<imp::ZoomControls>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ZoomControls {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
