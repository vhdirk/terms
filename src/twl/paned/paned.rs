use std::cell::{Cell, RefCell};
use std::marker::PhantomData;

use adw::subclass::prelude::*;
use glib::subclass::types::InterfaceList;
use glib::Properties;
use glib::{prelude::*, HasParamSpec};
use once_cell::sync::Lazy;
use tracing::warn;
use vte::WidgetExt;
use vte::{BoxExt, OrientableExt};

#[derive(Debug, Properties)]
#[properties(wrapper_type=super::Paned)]
pub struct Paned {
    container: gtk::Box,

    #[property(get=Self::get_orientation, set=Self::set_orientation, construct, builder(gtk::Orientation::Horizontal))]
    orientation: PhantomData<gtk::Orientation>,
}

impl Default for Paned {
    fn default() -> Self {
        Self {
            container: gtk::Box::new(gtk::Orientation::Horizontal, 0),
            orientation: Default::default(),
        }
    }
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

    fn append(&self, child: &impl glib::IsA<gtk::Widget>) {}
}
