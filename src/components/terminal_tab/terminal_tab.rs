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

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal_tab.ui")]
#[properties(wrapper_type = super::TerminalTab)]
pub struct TerminalTab {
    /// The working directory of the currently active terminal
    #[property(get, set, construct, nullable)]
    directory: RefCell<Option<PathBuf>>,

    /// The foreground command of the currently active terminal
    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(set, get, construct_only, nullable)]
    env: RefCell<Option<EnvMap>>,

    #[property(get, set, construct, nullable)]
    title: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalTab {
    const NAME: &'static str = "TermsTerminalTab";
    type Type = super::TerminalTab;
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
impl ObjectImpl for TerminalTab {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("close").build()]);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for TerminalTab {}
impl BinImpl for TerminalTab {}

#[gtk::template_callbacks]
impl TerminalTab {
    fn setup_widgets(&self) {
        let panel = TerminalPanel::new(self.directory.borrow().clone(), self.command.borrow().clone(), self.env.borrow().clone());
        self.obj().set_child(Some(&panel));

        // TODO
        panel.connect_exit(clone!(@weak self as this => move |panel| {
            this.obj().emit_by_name::<()>("close", &[]);
        }));

        // TODO: Only do this for active panel
        panel.connect_directory_notify(clone!(@weak self as this => move |p| {
            this.update_directory(p.directory());
        }));

        // TODO: Only do this for active panel
        panel.connect_title_notify(clone!(@weak self as this => move |p| {
            this.update_title(p.title());
        }));
    }

    fn update_title(&self, terminal_title: Option<String>) {
        // TODO: substitutions
        // ${activeTerminalTitle} 	The title of the current terminal with all variables substituted.
        // ${terminalCount} 	The total number of terminals in the session
        // ${terminalNumber} 	The number of the currently active terminal
        *self.title.borrow_mut() = terminal_title;
        self.obj().notify_title();
    }

    fn update_directory(&self, terminal_directory: Option<PathBuf>) {
        *self.directory.borrow_mut() = terminal_directory;
        self.obj().notify_directory();
    }
}
