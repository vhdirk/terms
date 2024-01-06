use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{gettext, LocaleCategory};

mod application;
mod components;
mod config;
mod services;
mod utils;

use application::Application;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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

fn init_gettext() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Unable to set the text domain encoding");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

fn init_resource() {
    gtk::Window::set_default_icon_name(APP_ID);
    glib::set_application_name(&gettext("Terms"));
    gio::resources_register_include!("resources.gresource").expect("Could not initialize resources");
}

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(EnvFilter::from_default_env()))
        .init();

    gtk::init().expect("Could not initialize gtk");
    // adw::init().expect("Could not initialize libadwaita");
    // panel::init();

    init_gettext();
    init_resource();
}

fn main() -> glib::ExitCode {
    init();

    // Create a new application
    let app = Application::new();

    // Run the application
    app.run()
}
