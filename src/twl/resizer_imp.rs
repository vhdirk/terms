/// This file is work derived from libpanel
/// https://gitlab.gnome.org/GNOME/libpanel/-/blob/10f7cedccc2577274d1ef5aa738b094edfad5d1e/src/panel-resizer.c
///
/// Copyright 2021 Christian Hergert <chergert@redhat.com>
/// libpanel is licensed GNU GPLv3
///
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{self, prelude::*, Properties};
use gtk::prelude::*;
use once_cell::sync::Lazy;
use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::Resizer)]
pub struct Resizer {
    #[property(get=Self::get_orientation, set=Self::set_orientation, construct, builder(gtk::Orientation::Horizontal))]
    orientation: PhantomData<gtk::Orientation>,

    handle: gtk::Separator,
}

#[glib::object_subclass]
impl ObjectSubclass for Resizer {
    const NAME: &'static str = "TwlResizer";
    type Type = super::Resizer;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for Resizer {
    fn constructed(&self) {
        self.parent_constructed();
        self.setup();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![]);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for Resizer {}

impl Resizer {
    fn setup(&self) {
        self.handle.set_parent(&*self.obj());
    }

    fn get_orientation(&self) -> gtk::Orientation {
        self.handle.orientation()
    }

    fn set_orientation(&self, orientation: gtk::Orientation) {
        self.handle.set_orientation(orientation)
    }
}
