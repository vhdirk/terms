use std::marker::PhantomData;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{self, Properties};

use super::utils::twl_widget_compute_expand;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::PackBox)]
pub struct PackBox {
    container: gtk::Box,
    start: gtk::Box,
    center: gtk::Box,
    end: gtk::Box,

    #[property(get=Self::get_orientation, set=Self::set_orientation, construct, explicit_notify, builder(gtk::Orientation::Horizontal))]
    orientation: PhantomData<gtk::Orientation>,
}

#[glib::object_subclass]
impl ObjectSubclass for PackBox {
    const NAME: &'static str = "TwlPackBox";
    type Type = super::PackBox;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Buildable, gtk::Orientable);

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.set_css_name("pack_box");
    }
}

#[glib::derived_properties]
impl ObjectImpl for PackBox {
    fn constructed(&self) {
        self.parent_constructed();

        self.container.set_parent(self.obj().as_ref());
        self.obj().set_focus_child(Some(&self.container));

        self.container.set_hexpand(true);
        self.container.set_vexpand(true);

        self.container.append(&self.start);
        self.container.append(&self.center);
        self.container.append(&self.end);

        self.center.set_hexpand(true);
        self.center.set_vexpand(true);
        self.apply_orientation();
    }

    fn dispose(&self) {
        self.container.unparent();
    }
}

impl WidgetImpl for PackBox {
    fn compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        twl_widget_compute_expand(&self.container, hexpand, vexpand);
    }
}

impl OrientableImpl for PackBox {}

impl BuildableImpl for PackBox {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        match (child.downcast_ref::<gtk::Widget>(), type_) {
            (Some(widget), Some(wtype)) if wtype == "start" => self.pack_start(widget),
            (Some(widget), Some(wtype)) if wtype == "end" => self.pack_end(widget),
            (Some(widget), _) => self.append(widget),
            (_, _) => self.parent_add_child(builder, child, type_),
        }
    }
}

impl PackBox {
    fn set_orientation(&self, orientation: gtk::Orientation) {
        if self.get_orientation() == orientation {
            return;
        }

        self.container.set_orientation(orientation);

        self.apply_orientation();
        self.obj().notify_orientation();
    }

    fn get_orientation(&self) -> gtk::Orientation {
        self.container.orientation()
    }

    fn apply_orientation(&self) {
        let orientation = self.get_orientation();
        let is_horizontal = orientation == gtk::Orientation::Horizontal;

        self.start.set_orientation(orientation);
        self.start.set_hexpand(!is_horizontal);
        self.start.set_vexpand(is_horizontal);

        self.center.set_orientation(orientation);

        self.end.set_orientation(orientation);
        self.end.set_hexpand(!is_horizontal);
        self.end.set_vexpand(is_horizontal);
    }

    pub fn pack_start(&self, widget: &impl IsA<gtk::Widget>) {
        self.start.append(widget);
    }

    pub fn append(&self, widget: &impl IsA<gtk::Widget>) {
        self.center.append(widget);
    }

    pub fn pack_end(&self, widget: &impl IsA<gtk::Widget>) {
        self.end.append(widget);
    }
}
