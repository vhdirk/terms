use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use glib::{clone, subclass::Signal};
use gtk::glib;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::path::PathBuf;
use tracing::info;

use crate::components::Terminal;
use crate::twl::{Panel, PanelGrid};
use crate::util::EnvMap;

#[derive(Debug, CompositeTemplate, Properties)]
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

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<String>>,

    #[template_child]
    panel_grid: TemplateChild<PanelGrid>,

    #[property(get=Self::get_selected, set=Self::set_selected, construct, nullable)]
    selected: PhantomData<Option<Terminal>>,

    selected_panel_signals: glib::SignalGroup,
    active_term_signals: glib::SignalGroup,
    active_term_bindings: glib::BindingGroup,
}

impl Default for TerminalTab {
    fn default() -> Self {
        Self {
            directory: Default::default(),
            command: Default::default(),
            env: Default::default(),
            title: Default::default(),
            icon: Default::default(),
            panel_grid: Default::default(),
            selected: Default::default(),

            selected_panel_signals: glib::SignalGroup::new::<Panel>(),
            active_term_signals: glib::SignalGroup::new::<Terminal>(),
            active_term_bindings: glib::BindingGroup::new(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalTab {
    const NAME: &'static str = "TermsTerminalTab";
    type Type = super::TerminalTab;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        PanelGrid::ensure_type();
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
        let term = Terminal::new(self.directory.borrow().clone(), self.command.borrow().clone(), self.env.borrow().clone());
        self.panel_grid.set_child(&term);

        // TODO
        term.connect_exit(clone!(@weak self as this => move |term, code| {
            this.obj().emit_by_name::<()>("close", &[]);
        }));

        // TODO: Only do this for active panel
        term.connect_directory_notify(clone!(@weak self as this => move |p| {
            this.update_directory(p.directory());
        }));

        // TODO: Only do this for active panel
        term.connect_title_notify(clone!(@weak self as this => move |p| {
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

    fn get_selected(&self) -> Option<Terminal> {
        self.panel_grid.selected_panel().and_then(|p| p.child()).and_downcast()
    }

    fn set_selected(&self, terminal: Option<Terminal>) {
        // todo!();
        // self.panel_grid.set_select
    }

    pub fn split(&self, orientation: Option<gtk::Orientation>) {
        let term = Terminal::new(self.directory.borrow().clone(), self.command.borrow().clone(), self.env.borrow().clone());

        self.panel_grid.split(&term, orientation);
    }
}
