use adw::prelude::BinExt;
use adw::subclass::prelude::*;
use glib::{clone, subclass::Signal};
use glib::{ObjectExt, Properties};
use gtk::glib;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::path::PathBuf;
use tracing::info;

use crate::components::terminal_panel::TerminalPanel;
use crate::util::EnvMap;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/zoom_controls.ui")]
pub struct ZoomControls {}

#[glib::object_subclass]
impl ObjectSubclass for ZoomControls {
    const NAME: &'static str = "TermsZoomControls";
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

impl ObjectImpl for ZoomControls {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("zoom-in").build()]);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for ZoomControls {}
impl BinImpl for ZoomControls {}

#[gtk::template_callbacks]
impl ZoomControls {
    fn setup_widgets(&self) {}
}
