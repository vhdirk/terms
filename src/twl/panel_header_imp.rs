use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{self, clone, Properties, WeakRef};
use gtk::{template_callbacks, CompositeTemplate};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;

use super::utils::TwlWidgetExt;
use super::{Bin, FadingLabel, PackBox, Panel};

// #[derive(Debug, Default)]
// pub struct Header {}

// #[glib::object_interface]
// unsafe impl ObjectInterface for Header {
//     const NAME: &'static str = "TwlHeader";
// }

#[derive(Debug, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Twl/gtk/panel_header.ui")]
#[properties(wrapper_type=super::PanelHeader)]
pub struct PanelHeader {
    #[property(get, set=Self::set_title, construct, nullable)]
    title: RefCell<Option<String>>,

    #[property(get=Self::get_title_widget, set=Self::set_title_widget, construct, nullable)]
    title_widget: PhantomData<Option<gtk::Widget>>,

    #[template_child]
    container: TemplateChild<PackBox>,

    #[template_child]
    title_container: TemplateChild<adw::Bin>,

    #[property(construct_only)]
    panel: WeakRef<Panel>,
}

impl Default for PanelHeader {
    fn default() -> Self {
        Self {
            title: Default::default(),
            title_widget: Default::default(),

            container: Default::default(),
            title_container: Default::default(),
            panel: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PanelHeader {
    const NAME: &'static str = "TwlPanelHeader";
    type Type = super::PanelHeader;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.set_css_name("panel_header");
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for PanelHeader {
    fn constructed(&self) {
        self.parent_constructed();
        self.setup();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("close").build()]);
        SIGNALS.as_ref()
    }

    fn dispose(&self) {
        self.container.unparent()
    }
}

impl WidgetImpl for PanelHeader {
    fn request_mode(&self) -> gtk::SizeRequestMode {
        self.container.request_mode()
    }
}

#[gtk::template_callbacks]
impl PanelHeader {
    fn setup(&self) {}

    fn construct_title_label(&self, title: Option<&str>) -> FadingLabel {
        let label = FadingLabel::new(title);
        label.add_css_class("title");
        label.set_valign(gtk::Align::Center);
        label
    }

    fn get_title_widget(&self) -> Option<gtk::Widget> {
        self.title_container.child()
    }

    fn set_title_widget(&self, widget: Option<&gtk::Widget>) {
        if self.title_container.child().as_ref() == widget {
            return;
        }

        self.title_container.set_child(widget);

        self.obj().notify_title_widget();
        self.obj().notify_title();
    }

    fn set_title(&self, title: Option<&str>) {
        *self.title.borrow_mut() = title.map(ToString::to_string);
        match title {
            Some(title) => {
                self.set_title_widget(Some(self.construct_title_label(Some(title)).upcast_ref()));
            },
            None => {
                self.set_title_widget(None::<&gtk::Widget>);
            },
        }
    }

    #[template_callback]
    fn on_close_clicked(&self) {
        if let Some(panel) = self.panel.upgrade() {
            panel.emit_by_name::<()>("close", &[]);
        }
    }
}
