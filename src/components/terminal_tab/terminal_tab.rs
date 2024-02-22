use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use glib::{clone, subclass::Signal};
use gtk::glib;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::path::PathBuf;
use tracing::*;

use crate::components::Terminal;
use crate::settings::Settings;
use crate::twl::{Panel, PanelGrid};
use crate::util::EnvMap;

const PANEL_HEADER_SMALL_CSS_CLASS: &str = "small";

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::TerminalTab)]
pub struct TerminalTab {
    settings: Settings,

    /// The working directory of the currently active terminal
    #[property(get, set, construct, nullable)]
    directory: RefCell<Option<PathBuf>>,

    /// The foreground command of the currently active terminal
    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(set, get, construct_only, nullable)]
    env: RefCell<Option<EnvMap>>,

    #[property(get, set, construct, nullable, explicit_notify)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<String>>,

    #[property(get=Self::get_selected, nullable)]
    selected: PhantomData<Option<Terminal>>,

    panel_grid: PanelGrid,
    selected_panel_signals: glib::SignalGroup,
    selected_terminal_signals: glib::SignalGroup,
    selected_terminal_bindings: glib::BindingGroup,
}

impl Default for TerminalTab {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            directory: Default::default(),
            command: Default::default(),
            env: Default::default(),
            title: Default::default(),
            icon: Default::default(),
            selected: Default::default(),

            panel_grid: PanelGrid::new(),
            selected_panel_signals: glib::SignalGroup::new::<Panel>(),
            selected_terminal_signals: glib::SignalGroup::new::<Terminal>(),
            selected_terminal_bindings: glib::BindingGroup::new(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalTab {
    const NAME: &'static str = "TermsTerminalTab";
    type Type = super::TerminalTab;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
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

    fn dispose(&self) {
        self.panel_grid.unparent();
    }
}

impl WidgetImpl for TerminalTab {
    fn grab_focus(&self) -> bool {
        self.panel_grid.grab_focus()
        // self.panel_grid.try_get().map(|p| p.grab_focus()).unwrap_or(false)
    }
}
impl BinImpl for TerminalTab {}

#[gtk::template_callbacks]
impl TerminalTab {
    fn setup_widgets(&self) {
        self.panel_grid.set_parent(&*self.obj());

        self.selected_terminal_signals.connect_notify_local(
            Some("title"),
            clone!(@weak self as this => move|obj, _|{
                this.update_title(obj.property("title"));
            }),
        );

        self.selected_terminal_signals.connect_notify_local(
            Some("directory"),
            clone!(@weak self as this => move |obj, _| {
                info!("active term: directory");
                this.directory.set(obj.property("directory"));
            }),
        );

        self.selected_panel_signals.connect_bind_local(clone!(@weak self as this => move |_sg, obj| {
            info!("selected page: bind");
            let panel = obj.downcast_ref::<Panel>();
            let term_obj = panel.map(Panel::child);
            let term = term_obj.and_downcast_ref::<Terminal>();

            info!("Selected terminal: {:?}", term);

            this.selected_terminal_signals.set_target(term);
            this.selected_terminal_bindings.set_source(term);
        }));

        self.panel_grid
            .connect_panel_close(clone!(@weak self as this => @default-return glib::Propagation::Proceed, move |_grid, panel| {
                this.on_panel_close_request(panel)
            }));

        self.panel_grid.connect_n_panels_notify(clone!(@weak self as this => move |_| {
            this.on_num_panels_changed()
        }));

        self.settings.bind_use_wide_panel_resize_handle(&self.panel_grid, "wide-handle").build();
        self.settings.connect_small_panel_headers_changed(clone!(@weak self as this => move |s| {
            this.set_small_panel_headers(s.small_panel_headers());
        }));
        self.set_small_panel_headers(self.settings.small_panel_headers());

        self.settings.bind_show_panel_headers(&self.panel_grid, "show-panel-headers").build();

        self.panel_grid.connect_selected_notify(clone!(@weak self as this => move |_| {
            this.on_selected_panel_change();
        }));

        let term = Terminal::new(self.directory.borrow().clone(), self.command.borrow().clone(), self.env.borrow().clone());
        let panel = self.panel_grid.set_initial_child(&term);

        self.connect_terminal_signals(&term, &panel);

        self.panel_grid.set_selected(Some(&panel));
    }

    fn update_title(&self, terminal_title: Option<String>) {
        // TODO: substitutions
        // ${activeTerminalTitle} 	The title of the current terminal with all variables substituted.
        // ${terminalCount} 	The total number of terminals in the session
        // ${terminalNumber} 	The number of the currently active terminal
        self.title.set(terminal_title);
        self.obj().notify_title();
    }

    fn get_selected(&self) -> Option<Terminal> {
        self.panel_grid.selected().map(|p| p.child()).and_downcast()
    }

    fn set_selected(&self, terminal: Option<Terminal>) {
        // todo!();
        // self.panel_grid.set_select
    }

    fn on_selected_panel_change(&self) {
        let panel = self.panel_grid.selected();
        debug!("on panel changed: {:?}", panel);
        self.selected_panel_signals.set_target(panel.as_ref());

        if let Some(term) = panel.as_ref().map(Panel::child).and_downcast::<Terminal>() {
            debug!("Set active term {:?}", term);
            term.grab_focus();
        }
    }

    pub fn split(&self, orientation: Option<gtk::Orientation>) {
        let term = Terminal::new(self.directory.borrow().clone(), self.command.borrow().clone(), self.env.borrow().clone());
        // term.grab_focus();

        let panel = self.panel_grid.split(&term, orientation);
        self.panel_grid.set_selected(Some(&panel));
        self.connect_terminal_signals(&term, &panel);
    }

    fn connect_terminal_signals(&self, terminal: &Terminal, panel: &Panel) {
        if self.settings.small_panel_headers() {
            panel.header().add_css_class(PANEL_HEADER_SMALL_CSS_CLASS);
        }

        terminal.connect_exit(clone!(@weak self as this, @weak panel as panel => move |term, code| {
            info!("Terminal {:?} exited with code {:?}", term, code);
            this.panel_grid.close_panel(&panel);
        }));

        terminal.bind_property("title", &panel.header(), "title").sync_create().build();
    }

    pub fn on_panel_close_request(&self, panel: &Panel) -> glib::Propagation {
        info!("on_panel_close_request: {:?}", panel);
        // TODO: test if process is still running
        if let Some(terminal) = panel.child().downcast_ref::<Terminal>() {}

        glib::Propagation::Proceed
    }

    pub fn on_num_panels_changed(&self) {
        let n_panels = self.panel_grid.n_panels();
        info!("on_num_panels_changed: {:?}", n_panels);
        if n_panels == 0 {
            info!("emit close signal");
            self.obj().emit_by_name::<()>("close", &[]);
        }
    }

    fn set_small_panel_headers(&self, small: bool) {
        for panel_header in self.panel_grid.panels().iter().map(|p| p.header()) {
            if small {
                panel_header.add_css_class(PANEL_HEADER_SMALL_CSS_CLASS);
            } else {
                panel_header.remove_css_class(PANEL_HEADER_SMALL_CSS_CLASS);
            }
        }
    }
}
