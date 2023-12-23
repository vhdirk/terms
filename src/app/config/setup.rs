// use super::{settings};
use super::{appearance, resources, actions, gettext, localization};

use anyhow::Result;
use relm4::gtk::gio::ApplicationFlags;
use relm4::gtk::prelude::{ApplicationExt, ApplicationExtManual};
use relm4::{gtk, main_adw_application};

pub fn init() -> Result<()> {
	gtk::init()?;
	gettext::init();
	localization::init();
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	relm4_icons::initialize_icons();
	actions::init();
	connect_signals();
	Ok(())
}


pub fn connect_signals() {
	let app = main_adw_application();
	app.set_flags(ApplicationFlags::HANDLES_OPEN);
	app.connect_open(|_, _, _| {});
}

pub fn init_services() -> Result<()> {
	// settings::init()?;
	appearance::init()
}

pub fn refresh() -> Result<()> {
	// settings::refresh()
	Ok(())
}

