use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;

use crate::components::PreferencesWindow;
use crate::config::PROFILE;
use crate::settings::Settings;
use crate::util::EnvMap;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/window.ui")]
pub struct Window {
    pub settings: Settings,
    pub init_args: RefCell<TerminalInitArgs>,

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub overlay: TemplateChild<gtk::Overlay>,

    #[template_child]
    pub container: TemplateChild<gtk::Box>,

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

        if PROFILE.should_use_devel_class() {
            let obj = self.obj();
            obj.add_css_class("devel");
        }

        self.setup_widgets();
        self.setup_gactions();
        self.connect_signals();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

impl Window {
    fn setup_widgets(&self) {
        self.header_bar.set_container(Some(&*self.container));
        self.header_bar.set_overlay(Some(&*self.overlay));

        if self.settings.remember_window_size() {
            self.load_window_size();
        }

        // self.new_session(Some(self.init_args.borrow().clone()));
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

        self.obj().connect_fullscreened_notify(clone!(@weak self as this => move |w| {
            this.header_bar.set_fullscreened(w.is_fullscreened());
        }));
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::ActionEntry::builder("edit-preferences")
            .activate(clone!(@weak self as this => move |_win: &super::Window, _, _| {
                this.open_preferences();
            }))
            .build();

        let new_session_action = gio::ActionEntry::builder("new-session")
            .activate(clone!(@weak self as this => move |_win: &super::Window, _, _| {
                this.new_session(None);
            }))
            .build();

        let toggle_fullscreen_action = gio::ActionEntry::builder("toggle-fullscreen")
            .activate(move |win: &super::Window, _, _| win.set_fullscreened(!win.is_fullscreened()))
            .build();

        self.obj()
            .add_action_entries([preferences_action, new_session_action, toggle_fullscreen_action]);
    }

    pub fn open_preferences(&self) {
        let prefs_window = PreferencesWindow::new(Some(self.obj().as_ref()));
        prefs_window.set_visible(true);
    }

    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args.clone();

        self.new_session(Some(init_args.clone()));
    }

    pub fn new_session(&self, init_args: Option<TerminalInitArgs>) {
        let command = init_args.as_ref().and_then(|a| a.command.clone());
        let working_directory = init_args.as_ref().and_then(|a| a.working_dir.clone());
        let env = init_args.as_ref().map(|a| EnvMap::from(a.env.clone()));

        let session = Session::new(working_directory, command, env);
        self.tab_view.append(&session);

        session.connect_close(clone!(@weak self as this => move |session: &Session| {
            this.tab_view.close_page(&this.tab_view.page(session));

            if this.tab_view.n_pages() == 0 {
                    this.obj().close();
            }
        }));
    }
}
