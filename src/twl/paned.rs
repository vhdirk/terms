use glib::{prelude::*, subclass::prelude::*};

use super::paned_imp as imp;

glib::wrapper! {
        pub struct Paned(ObjectSubclass<imp::Paned>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::AccessibleRange;
}

impl Paned {
    pub fn new(orientation: gtk::Orientation) -> Self {
        glib::Object::builder().property("orientation", orientation).build()
    }

    pub fn replace(&self, child: Option<&impl IsA<gtk::Widget>>, new_child: Option<&impl IsA<gtk::Widget>>) {
        self.imp().replace(
            child.and_then(|w| w.clone().dynamic_cast().ok()),
            new_child.and_then(|w| w.clone().dynamic_cast().ok()),
        );
    }

    pub fn sibling(&self, child: Option<&impl IsA<gtk::Widget>>) -> Option<gtk::Widget> {
        self.imp().sibling(child.and_then(|w| w.clone().dynamic_cast().ok()))
    }
}
