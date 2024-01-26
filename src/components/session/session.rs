use adw::subclass::prelude::*;
use glib::{clone, subclass::Signal};
use glib::{ObjectExt, Properties};
use gtk::glib;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::path::PathBuf;
use tracing::info;

use super::TerminalInitArgs;
use crate::components::terminal_frame::TerminalFrame;
use crate::util::EnvMap;

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/session.ui")]
#[properties(wrapper_type = super::Session)]
pub struct Session {
    pub init_args: RefCell<TerminalInitArgs>,

    /// The working directory of the currently active terminal
    #[property(get, set, construct, nullable)]
    working_directory: RefCell<Option<PathBuf>>,

    /// The foreground command of the currently active terminal
    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(set, get, construct_only, nullable)]
    env: RefCell<Option<EnvMap>>,
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

#[glib::derived_properties]
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
    fn setup_widgets(&self) {
        let panel = TerminalFrame::new(
            self.working_directory.borrow().clone(),
            self.command.borrow().clone(),
            self.env.borrow().clone(),
        );
        self.obj().set_property("child", &panel);

        panel.connect_exit(clone!(@weak self as this => move |panel| {
            this.obj().emit_by_name::<()>("close", &[]);
        }));

        panel.connect_command_notify(move |p| {
            info!("Terminal panel command changed: {:?}", p.command());
        });

        panel.connect_working_directory_notify(move |p| {
            info!("Terminal panel working dir changed: {:?}", p.working_directory());
        });
    }
}
