use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{gettext, LocaleCategory};
use serde::{Deserialize, Serialize};

mod application;
mod components;
mod config;
mod services;
mod utils;

use application::Application;

// #[doc(hidden)]
// pub fn resources_register_include_impl(bytes: &'static [u8]) -> Result<(), glib::Error> {
//     let bytes = glib::Bytes::from_static(bytes);
//     let resource = Resource::from_data(&bytes)?;
//     resources_register(&resource);
//     Ok(())
// }

// // rustdoc-stripper-ignore-next
// /// Include gresources generated with `glib_build_tools::compile_resources` and register with glib. `path` is
// /// relative to `OUTDIR`.
// ///
// /// ```ignore
// /// gio::resources_register_include!("compiled.gresource").unwrap();
// /// ```
// #[macro_export]
// macro_rules! resources_register_include {
//     ($path:expr) => {
//         $crate::resources_register_include_impl(include_bytes!(concat!(
//             env!("OUT_DIR"),
//             "/",
//             $path
//         )))
//     };
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ColorScheme {
    Dark,
    Light,
    Default,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::Default
    }
}

fn init_gettext() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Unable to set the text domain encoding");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

fn init_resource() -> Result<(), glib::Error> {
    glib::set_application_name(&gettext("Terms"));
    gio::resources_register_include!("resources.gresource")?;
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/io/github/vhdirk/Terms/gtk/style.css");
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
    gtk::Window::set_default_icon_name(APP_ID);
    Ok(())
}

pub fn init() -> Result<(), glib::Error> {
    gtk::init().expect("Could not initialize gtk");
    adw::init().expect("Could not initialize libadwaita");
    panel::init();

    init_gettext();
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();
    init_resource()?;
    Ok(())
}

fn main() -> glib::ExitCode {
    init().expect("Could not initialize");
    // Create a new application
    let app = Application::new();

    // Run the application
    app.run()
}
