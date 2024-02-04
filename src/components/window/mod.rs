mod window;
use std::path::PathBuf;

use glib::{prelude::*, subclass::prelude::*};
use tracing::*;
use window as imp;

use crate::util::EnvMap;

glib::wrapper! {
        pub struct Window(ObjectSubclass<imp::Window>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window, adw::ApplicationWindow, //, panel::Workspace,
                @implements gtk::Accessible, gio::ActionGroup, gio::ActionMap, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder().property("application", application).build()
    }

    pub fn new_tab(&self, command: Option<String>, directory: Option<PathBuf>, env: Option<EnvMap>) {
        self.imp().new_tab(command, directory, env);
    }

    pub fn transfer_tab(&self, view: &adw::TabView, page: &adw::TabPage, position: i32) {
        let our_view = &*self.imp().tab_view;
        debug!("Transferring tab page {:?} from {:?} to {:?}", page, view, our_view);
        view.transfer_page(page, our_view, position);
    }

    pub fn tab_view(&self) -> Option<adw::TabView> {
        self.imp().tab_view.try_get()
    }
}
