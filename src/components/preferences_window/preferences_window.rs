use adw::ffi::AdwPreferencesWindow;
use adw::subclass::prelude::*;
use glib::{clone, closure_local, RustClosure};
use gtk::prelude::*;
use gtk::{gio, glib};
use panel::subclass::prelude::*;
use std::cell::RefCell;

use super::*;

// var builder = new Gtk.Builder.from_resource ("/com/raggesilver/BlackBox/gtk/tab-menu.ui");
// this.tab_view.menu_model = builder.get_object ("tab-menu") as GLib.Menu;

// this.layout_box.append (this.header_bar_revealer);
// this.layout_box.append (this.tab_view);

// this.overlay = new Gtk.Overlay ();
// this.overlay.child = this.layout_box;

// this.content = this.overlay;

// this.set_name ("blackbox-main-window");

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/preferences_window.ui")]
pub struct PreferencesWindow {}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesWindow {
    const NAME: &'static str = "TermsPreferencesWindow";
    type Type = super::PreferencesWindow;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }
}

impl WidgetImpl for PreferencesWindow {}
impl WindowImpl for PreferencesWindow {}
impl AdwWindowImpl for PreferencesWindow {}
impl PreferencesWindowImpl for PreferencesWindow {}
// impl WorkspaceImpl for PreferencesWindow {}

impl PreferencesWindow {
    fn setup_widgets(&self) {
        self.connect_signals();
    }

    fn connect_signals(&self) {}
}
