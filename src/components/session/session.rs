use adw::subclass::prelude::*;
use glib::ObjectExt;
use glib::{clone, subclass::Signal};
use gtk::glib;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::RefCell;

use super::TerminalInitArgs;
use crate::components::terminal_frame::TerminalFrame;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/session.ui")]
// #[properties(wrapper_type = super::Session)]
pub struct Session {
    pub init_args: RefCell<TerminalInitArgs>,
}

#[glib::object_subclass]
impl ObjectSubclass for Session {
    const NAME: &'static str = "TermsSession";
    type Type = super::Session;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// #[glib::derived_properties]
impl ObjectImpl for Session {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("close").build()]);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for Session {}
impl BinImpl for Session {}

#[gtk::template_callbacks]
impl Session {
    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }

    fn setup_widgets(&self) {
        let panel = TerminalFrame::new(self.init_args.borrow().clone());
        self.obj().set_property("child", &panel);

        panel.connect_exit(clone!(@weak self as this => move |panel| {
                            this.obj().emit_by_name::<()>("close", &[]);
        }));
    }
}
