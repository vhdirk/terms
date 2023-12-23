use anyhow::{Ok, Result};
use gettextrs::gettext;
use relm4::adw::{gdk, gio};
use relm4::gtk::{self, glib};

use super::info::APP_ID;

pub(crate) fn init() -> Result<()> {
	glib::set_application_name(&gettext("Terms"));
	gio::resources_register_include!("resources.gresource")?;
	let provider = gtk::CssProvider::new();
	provider.load_from_resource("/com/github/vhdirk/Terms/ui/style.css");
	if let Some(display) = gdk::Display::default() {
		gtk::style_context_add_provider_for_display(
			&display,
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
	}
	gtk::Window::set_default_icon_name(APP_ID);
	Ok(())
}
