use std::{cell::RefCell, marker::PhantomData};

use adw::subclass::prelude::*;
use glib::prelude::*;
use glib::Properties;
use gtk::prelude::*;
use tracing::warn;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::Paned)]
pub struct Paned {
    container: gtk::Box,

    #[property(get=Self::get_orientation, set=Self::set_orientation, construct, builder(gtk::Orientation::Horizontal))]
    orientation: PhantomData<gtk::Orientation>,

    children: RefCell<Vec<gtk::Widget>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Paned {
    const NAME: &'static str = "TwlPaned";
    type Type = super::Paned;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for Paned {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup();
    }
}
impl WidgetImpl for Paned {}

impl OrientableImpl for Paned {}

impl BuildableImpl for Paned {
    fn add_child(&self, _builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        match child.downcast_ref::<gtk::Widget>() {
            Some(widget) => self.append(widget),
            None => warn!("Cannot add child of type {:?} to {:?}", type_, self.obj()),
        }
    }
}

impl Paned {
    fn setup(&self) {
        self.container.set_parent(&*self.obj());
    }

    fn get_orientation(&self) -> gtk::Orientation {
        self.container.orientation()
    }

    fn set_orientation(&self, orientation: gtk::Orientation) {
        self.container.set_orientation(orientation)
    }

    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        self.insert(-1, child);
    }

    pub fn prepend(&self, child: &impl IsA<gtk::Widget>) {
        self.insert(0, child);
    }

    fn insert(&self, position: i32, child: &impl IsA<gtk::Widget>) {
        match position {
            -1 => (),
            0 => (),
            _ => (),
        }
    }
}
