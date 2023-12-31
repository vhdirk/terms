use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use panel::subclass::prelude::*;

use std::cell::RefCell;

use glib::{clone, closure_local, RustClosure};

use crate::components::{
    header_bar::HeaderBar,
    terminal::{Terminal, TerminalInitArgs},
    terminal_panel::TerminalPanel,
};

use super::*;

// var builder = new Gtk.Builder.from_resource ("/com/raggesilver/BlackBox/gtk/tab-menu.ui");
// this.tab_view.menu_model = builder.get_object ("tab-menu") as GLib.Menu;

// this.layout_box.append (this.header_bar_revealer);
// this.layout_box.append (this.tab_view);

// this.overlay = new Gtk.Overlay ();
// this.overlay.child = this.layout_box;

// this.content = this.overlay;

// this.set_name ("blackbox-main-Workspace");

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/workspace.ui")]
pub struct Workspace {
    pub init_args: RefCell<TerminalInitArgs>,

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Workspace {
    const NAME: &'static str = "TermsWorkspace";
    type Type = super::Workspace;
    type ParentType = panel::Workspace;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Workspace {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }
}

impl WidgetImpl for Workspace {}
impl WindowImpl for Workspace {}
impl ApplicationWindowImpl for Workspace {}
impl AdwApplicationWindowImpl for Workspace {}
impl WorkspaceImpl for Workspace {}

impl Workspace {
    fn setup_widgets(&self) {
        let panel = TerminalPanel::new(self.init_args.borrow().clone());
        self.tab_view.append(&panel);

        panel.connect_exit(clone!(@weak self as this => move |panel: &TerminalPanel| {
                                this.tab_view.close_page(&this.tab_view.page(panel));

                                if this.tab_view.n_pages() == 0 {
                                                this.obj().close();
                                }
        }));

        // panel.connect_closure(
        // 	"exit",
        // 	false,
        // 	RustClosure::new_local(clone!(@weak self as this, move |_terminal: TerminalPanel| {

        // 	})),
        // );

        self.connect_signals();
    }

    fn connect_signals(&self) {}

    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }
}
