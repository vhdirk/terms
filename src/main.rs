use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::LocaleCategory;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use application::Application;

mod application;
mod components;
mod config;
mod error;
mod i18n;
mod pcre2;
mod services;
mod settings;
mod theme_provider;
mod util;

fn init_gettext() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Unable to set the text domain encoding");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(EnvFilter::from_default_env()))
        .init();

    init_gettext();

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);
}

fn main() -> glib::ExitCode {
    init();

    let app = Application::new();
    app.run()
}
