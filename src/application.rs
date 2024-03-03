use adw::{prelude::*, subclass::prelude::*};
use gettextrs::gettext;
use gio::{self, ApplicationFlags};
use glib::{self, ExitCode};
use std::{cell::RefCell, path::PathBuf};
use std::{fmt, rc::Rc};
use tracing::*;

use crate::{
    components::Window,
    config::{self, APP_ID, APP_NAME, VERSION},
    settings::Settings,
    theme_provider::ThemeProvider,
    util::EnvMap,
};

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
    /// Whether this `AppProfile` should use the `.devel` CSS class on windows.
    pub fn should_use_devel_class(&self) -> bool {
        matches!(self, Self::Devel)
    }
}

impl From<&AppProfile> for &str {
    /// The string representation of this `AppProfile`.
    fn from(val: &AppProfile) -> Self {
        match *val {
            AppProfile::Stable => "stable",
            AppProfile::Beta => "beta",
            AppProfile::Devel => "devel",
        }
    }
}

impl fmt::Display for AppProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

mod imp {
    use glib::clone;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {
        pub settings: Settings,

        pub command: RefCell<Option<String>>,
        pub directory: RefCell<Option<PathBuf>>,
        pub env: RefCell<Option<EnvMap>>,
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

            self.setup_gactions();

            self.setup_shortcuts();
            self.setup_signals();

            self.setup_command_line();
        }
    }

    impl ApplicationImpl for Application {
        fn activate(&self) {
            // late-init the theme provider
            ThemeProvider::default();

            self.new_window(self.command.borrow().clone(), self.directory.borrow().clone(), self.env.borrow().clone());
        }

        fn startup(&self) {
            self.parent_startup();

            gtk::Window::set_default_icon_name(APP_ID);
            glib::set_application_name(&gettext("Terms"));
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

            let working_dir = options.lookup_value("working-directory", None).and_then(|w| w.get::<PathBuf>());

            let command = options.lookup_value("command", None).and_then(|w| w.get::<String>());

            *self.command.borrow_mut() = command;
            *self.directory.borrow_mut() = working_dir;

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

        pub fn new_window(&self, command: Option<String>, directory: Option<PathBuf>, env: Option<EnvMap>) {
            let app = self.obj();

            info!("Window init args: {:?} {:?} {:?}", command, directory, env);

            let window = Window::new(&*app);
            window.new_tab(command, directory, env);

            info!("Add window");
            app.add_window(&window);
            window.present();
        }

        fn setup_gactions(&self) {
            let quit_action = gio::ActionEntry::builder("quit")
                .activate(move |app: &super::Application, _, _| app.quit())
                .build();
            let about_action = gio::ActionEntry::builder("about")
                .activate(move |app: &super::Application, _, _| app.show_about())
                .build();
            let new_window = gio::ActionEntry::builder("new-window")
                .activate(move |app: &super::Application, _, _| app.new_window())
                .build();

            self.obj().add_action_entries([quit_action, about_action, new_window]);
        }

        fn setup_signals(&self) {
            for key in self.settings.shortcuts().keys() {
                self.settings.shortcuts().connect_changed(
                    Some(&key),
                    clone!(@weak self as this => move |_, key| {
                        let (action, accels) = this.settings.shortcuts().entry(&key);
                        this.obj()
                            .set_accels_for_action(&action, &accels.iter().map(|a| a.as_str()).collect::<Vec<_>>());
                    }),
                );
            }
        }

        fn setup_shortcuts(&self) {
            let shortcut_settings = self.settings.shortcuts();
            for (action, accels) in shortcut_settings.entries() {
                self.obj()
                    .set_accels_for_action(&action, &accels.iter().map(|a| a.as_str()).collect::<Vec<_>>())
            }
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
        Self::register_startup_hook(&app);
        app
    }

    pub fn run(&self) -> ExitCode {
        info!("Terms ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self)
    }

    fn new_window(&self) {
        // TODO: this fails if the last active window was not a 'Window' but perhaps preferences
        let directory = self.active_window().and_downcast::<Window>().and_then(|window| window.directory());

        // TODO: respect working_directory_mode setting
        self.imp().new_window(None, directory, None);
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
            .copyright("© 2023 Dirk Van Haerenborgh")
            .website("https://vhdirk.github.io/terms")
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

    pub(crate) fn register_startup_hook(app: &Self) {
        let signalid: Rc<RefCell<Option<glib::SignalHandlerId>>> = Rc::new(RefCell::new(None));
        {
            let signalid_ = signalid.clone();

            let id = app.connect_startup(move |app| {
                app.disconnect(signalid_.borrow_mut().take().expect("Signal ID went missing"));
                gtk::init().expect("Failed to initalize gtk4");
                adw::init().expect("Failed to initialize adw");
            });
            *signalid.borrow_mut() = Some(id);
        }
    }
}
