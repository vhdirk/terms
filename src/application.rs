use std::{borrow::Cow, fmt};

use crate::config::{self, APP_ID, APP_NAME, VERSION};
use adw;
use gettextrs::gettext;
use gio::{ApplicationFlags, Settings};
use glib::ExitCode;
use gtk::{gio, glib, glib::clone, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, path::PathBuf};
use tracing::*;

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

    use std::{collections::HashMap, env};

    use adw::subclass::prelude::AdwApplicationImpl;
    // use panel::{prelude::WorkbenchExt, subclass::prelude::PanelApplicationImpl};

    use crate::{
        components::{TerminalInitArgs, Window},
        config::APP_ID,
        services::theme_provider::ThemeProvider,
    };

    use super::*;

    #[derive(Debug)]
    pub struct Application {
        /// The application settings.
        pub settings: Settings,
        // pub theme_provider: ThemeProvider,
        // /// The system settings.
        // pub system_settings: SystemSettings,
        // /// The list of logged-in sessions.
        // pub session_list: SessionList,
        pub init_args: RefCell<TerminalInitArgs>,
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                settings: Settings::new(APP_ID),
                // system_settings: Default::default(),
                // session_list: Default::default(),
                init_args: RefCell::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "TermsApplication";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.parent_constructed();
            let app = self.obj();
            app.setup_gactions();
            app.set_accels_for_action("win.show-help-overlay", &["<Control>question"]);
            app.set_accels_for_action("win.edit-preferences", &["<Control>comma"]);

            self.setup_command_line();
        }
    }

    impl ApplicationImpl for Application {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            // init the theme provider
            ThemeProvider::default();
            let app = self.obj();
            // Get the current window or create one if necessary

            // TODO: keep track of windows

            let window = Window::new(app.as_ref(), self.init_args.borrow().clone());

            // Ask the window manager/compositor to present the window
            info!("Add window");
            app.add_window(&window);
            window.present();

            // let workbench = panel::Workbench::new();
            // let workspace = Workspace::new(&*app, self.init_args.borrow().clone());
            // workbench.add_workspace(&workspace);
        }

        fn startup(&self) {
            self.parent_startup();
        }

        fn command_line(&self, command_line: &gio::ApplicationCommandLine) -> ExitCode {
            let env = command_line.environ();
            info!("running with env {:?}", env);

            if !self.obj().is_remote() {
                self.activate();
            }

            self.parent_command_line(command_line)
        }

        fn handle_local_options(&self, options: &glib::VariantDict) -> ExitCode {
            info!("command options: ");

            if options.contains("version") {
                // Nothing to do here; Version is always printed
                return ExitCode::SUCCESS;
            }

            let working_dir = options
                .lookup_value("working-directory", None)
                .and_then(|w| w.get::<PathBuf>())
                .or(env::current_dir().ok());

            let command = options.lookup_value("command", None).and_then(|w| w.get::<String>());

            self.set_init_args(TerminalInitArgs {
                working_dir,
                command,
                env: HashMap::new(),
            });

            self.parent_handle_local_options(options)
        }
    }

    impl GtkApplicationImpl for Application {
        fn window_removed(&self, window: &gtk::Window) {
            self.parent_window_removed(window);
        }
    }
    impl AdwApplicationImpl for Application {}
    // impl PanelApplicationImpl for Application {}

    impl Application {
        fn setup_command_line(&self) {
            self.obj().add_main_option(
                "version",
                'v'.try_into().unwrap(),
                glib::OptionFlags::NONE,
                glib::OptionArg::None,
                &gettext("Show app version"),
                None,
            );
            self.obj().add_main_option(
                "working-directory",
                'w'.try_into().unwrap(),
                glib::OptionFlags::NONE,
                glib::OptionArg::Filename,
                &gettext("Set current working directory"),
                Some("CWD"),
            );
            self.obj().add_main_option(
                "command",
                'c'.try_into().unwrap(),
                glib::OptionFlags::NONE,
                glib::OptionArg::String,
                &gettext("Execute command in a terminal"),
                Some("CMD"),
            );
        }

        fn set_init_args(&self, init_args: TerminalInitArgs) {
            let mut args = self.init_args.borrow_mut();
            *args = init_args;
        }
    }
}

glib::wrapper! {
        pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for Application {
    fn default() -> Self {
        adw::Application::default().downcast().unwrap()
    }
}

impl Application {
    pub fn new() -> Self {
        let app: Self = glib::Object::builder()
            .property("application-id", Some(config::APP_ID))
            .property("flags", ApplicationFlags::SEND_ENVIRONMENT | ApplicationFlags::HANDLES_COMMAND_LINE)
            .property("resource-base-path", Some("/io/github/vhdirk/Terms"))
            .build();
        app.set_default();
        app
    }

    pub fn run(&self) -> ExitCode {
        info!("Terms ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self)
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit").activate(move |app: &Self, _, _| app.quit()).build();
        let about_action = gio::ActionEntry::builder("about").activate(move |app: &Self, _, _| app.show_about()).build();

        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = adw::AboutWindow::builder()
            .transient_for(&window)
            .version(VERSION)
            .icon_name(APP_ID)
            .application_icon(APP_ID)
            .application_name(APP_NAME)
            .license_type(gtk::License::Gpl30)
            .developer_name("Dirk Van Haerenborgh")
            .copyright("© 2022 Dirk Van Haerenborgh")
            .website("https://github.com/vhdirk/terms/")
            .issue_url("https://github.com/vhdirk/terms/issues")
            // .translator_credits(gettext("translator-credits").replace("\\n", "\n"))
            .developers(vec!["Dirk Van Haerenborgh <vhdirk@gmail.com>"])
            .artists(vec!["Dirk Van Haerenborgh <vhdirk@gmail.com>"])
            .documenters(vec!["Dirk Van Haerenborgh <vhdirk@gmail.com>"])
            .comments(gettext("A terminal where conditions apply."))
            .modal(true)
            .build();
        dialog.present();
    }
}
