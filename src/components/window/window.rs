use adw::subclass::prelude::*;
use gio::SimpleAction;
use glib::{clone, closure_local, RustClosure};
use gtk::prelude::*;
use gtk::{gio, glib};
// use panel::subclass::prelude::*;
use std::cell::RefCell;

use crate::components::PreferencesWindow;
use crate::services::settings::Settings;
use crate::services::theme_provider::ThemeProvider;

use super::*;

// var builder = new Gtk.Builder.from_resource ("/com/raggesilver/BlackBox/gtk/tab-menu.ui");
// this.tab_view.menu_model = builder.get_object ("tab-menu") as GLib.Menu;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/window.ui")]
pub struct Window {
    pub settings: Settings,
    pub init_args: RefCell<TerminalInitArgs>,

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TermsWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_gactions();
        self.setup_widgets();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

impl Window {
    fn setup_widgets(&self) {
        let session = Session::new(self.init_args.borrow().clone());
        self.tab_view.append(&session);

        session.connect_close(clone!(@weak self as this => move |session: &Session| {
                                this.tab_view.close_page(&this.tab_view.page(session));

                                if this.tab_view.n_pages() == 0 {
                                        this.obj().close();
                                }
        }));

        if self.settings.remember_window_size() {
            self.load_window_size();
        }

        self.connect_signals();
    }

    fn load_window_size(&self) {
        let obj = self.obj();
        obj.set_default_width(self.settings.window_width() as i32);
        obj.set_default_height(self.settings.window_height() as i32);
        obj.set_fullscreened(self.settings.was_fullscreened());
        obj.set_maximized(self.settings.was_maximized());
    }

    fn connect_signals(&self) {
        self.obj().connect_default_width_notify(clone!(@weak self as this => move |w| {
            this.settings.set_window_width(w.default_width() as u32);
        }));

        self.obj().connect_default_height_notify(clone!(@weak self as this => move |w| {
            this.settings.set_window_height(w.default_height() as u32);
        }));

        self.obj().connect_fullscreened_notify(clone!(@weak self as this => move |w| {
            this.settings.set_was_fullscreened(w.is_fullscreened());
        }));

        self.obj().connect_maximized_notify(clone!(@weak self as this => move |w| {
            this.settings.set_was_maximized(w.is_maximized());
        }));
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::ActionEntry::builder("edit-preferences")
            .activate(clone!(@weak self as this => move |_win: &super::Window, _, _| {
                this.open_preferences();
            }))
            .build();

        self.obj().add_action_entries([preferences_action]);
    }

    pub fn open_preferences(&self) {
        let prefs_window = PreferencesWindow::new(Some(self.obj().as_ref()));
        prefs_window.set_visible(true);
    }

    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }
}
