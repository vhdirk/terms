use std::{borrow::Cow, fmt};

use gettextrs::gettext;
use gio::{ApplicationFlags, Settings};
use glib::ExitCode;
use gtk::{gio, glib, glib::clone, prelude::*, subclass::prelude::*};
use serde::{Serialize, Deserialize};
use tracing::{debug, info};
use adw;
use crate::config::{self, APP_ID, VERSION};

// use crate::{config, session_list::SessionList, spawn, system_settings::SystemSettings, Window};

/// The profile that was built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppProfile {
    /// A stable release.
    Stable,
    /// A beta release.
    Beta,
    /// A development release.
    Devel,
}

impl AppProfile {
    /// The string representation of this `AppProfile`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Devel => "devel",
        }
    }

    /// Whether this `AppProfile` should use the `.devel` CSS class on windows.
    pub fn should_use_devel_class(&self) -> bool {
        matches!(self, Self::Devel)
    }
}

impl fmt::Display for AppProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

mod imp {
    use adw::subclass::prelude::AdwApplicationImpl;
    use panel::subclass::prelude::PanelApplicationImpl;

    use crate::{config::APP_ID, components::window::Window};

    use super::*;

    #[derive(Debug)]
    pub struct Application {
        // /// The application settings.
        // pub settings: Settings,
        // /// The system settings.
        // pub system_settings: SystemSettings,
        // /// The list of logged-in sessions.
        // pub session_list: SessionList,
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                // settings: Settings::new(APP_ID),
                // system_settings: Default::default(),
                // session_list: Default::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "TermsApplication";
        type Type = super::Application;
        type ParentType = panel::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.parent_constructed();
            let app = self.obj();
            app.setup_gactions();
            app.set_accels_for_action("win.show-help-overlay", &["<Control>question"]);
        }
    }

    impl ApplicationImpl for Application {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let app = self.obj();
            // Get the current window or create one if necessary

            let window = Window::new(&*app);

            // Ask the window manager/compositor to present the window
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();
        }
    }

    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
    impl PanelApplicationImpl for Application {}

}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application, panel::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for Application {
    fn default() -> Self {
        gio::Application::default()
            .and_downcast::<Application>()
            .unwrap()
    }
}

impl Application {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", Some(config::APP_ID))
            .property("flags", ApplicationFlags::default())
            .property("resource-base-path", Some("/com/github/vhdirk/Terms/"))
            .build()
    }

    pub fn run(&self) -> ExitCode {
        info!("Terms ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self)
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
		let dialog = adw::AboutWindow::builder()
            .transient_for(&window)
			.icon_name(APP_ID)
			.application_icon(APP_ID)
			.application_name("Terms")
			.developer_name("Dirk Van Haerenborgh")
			.website("Website")
			.copyright("© 2022 Dirk Van Haerenborgh")
			.license_type(gtk::License::Gpl30)
			.website("https://github.com/vhdirk/terms/")
			.issue_url("https://github.com/vhdirk/terms/issues")
			.version(VERSION)
			.translator_credits(gettext("translator-credits").replace("\\n", "\n"))
			.modal(true)
			.developers(vec![
				"Dirk Van Haerenborgh <vhdirk@gmail.com>",
			])
			.artists(vec![
				"Dirk Van Haerenborgh <vhdirk@gmail.com>",
			])
			.documenters(vec!["Dirk Van Haerenborgh <vhdirk@gmail.com>"])
			.comments("A terminal where conditions apply.")
			.build();
		dialog.present();
    }

}


