/// This file is work derived from Prompt
/// https://gitlab.gnome.org/chergert/prompt/-/blob/c279d3dbe78a126d4de732b3383aa3e8be3bafdb/src/prompt-window.c
///
/// Copyright 2023 Christian Hergert <chergert@redhat.com>
/// Prompt is licensed GNU GPLv3
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{self, Properties};
use gtk::prelude::*;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::Cell;

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Twl/gtk/zoom_controls.ui")]
#[properties(wrapper_type=super::ZoomControls)]
pub struct ZoomControls {
    /// The current zoom value, in percent
    #[property(get, set=Self::set_value, construct, default=100)]
    value: Cell<u32>,

    #[template_child]
    zoom_label: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for ZoomControls {
    const NAME: &'static str = "TwlZoomControls";
    type Type = super::ZoomControls;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for ZoomControls {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("zoom-in").build(),
                Signal::builder("zoom-out").build(),
                Signal::builder("zoom-reset").build(),
                Signal::builder("zoom").param_types([u32::static_type()]).build(),
            ]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for ZoomControls {}
impl BinImpl for ZoomControls {}

#[gtk::template_callbacks]
impl ZoomControls {
    fn set_value(&self, value: u32) {
        self.zoom_label.set_label(&format!("{}%", value));
    }

    #[template_callback]
    fn on_zoom_out_clicked(&self) {
        self.obj().emit_by_name::<()>("zoom-out", &[]);
    }

    #[template_callback]
    fn on_zoom_label_clicked(&self) {
        self.obj().emit_by_name::<()>("zoom-reset", &[]);
    }

    #[template_callback]
    fn on_zoom_in_clicked(&self) {
        self.obj().emit_by_name::<()>("zoom-in", &[]);
    }
}
