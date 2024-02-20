use glib::{prelude::*, subclass::prelude::*};

use super::{fading_label_imp as imp, utils::TwlWidgetExt};

glib::wrapper! {
        pub struct FadingLabel(ObjectSubclass<imp::FadingLabel>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FadingLabel {
    pub fn new(label: Option<&str>) -> Self {
        let mut builder = glib::Object::builder();
        if let Some(label) = label {
            builder = builder.property("label", label);
        }
        builder.build()
    }
}

impl Default for FadingLabel {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
impl TwlWidgetExt for FadingLabel {}
