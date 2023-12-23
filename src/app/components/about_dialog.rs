use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::{
	app::config::info::{APP_ID, VERSION},
	fl,
};

pub struct AboutDialog {}

pub struct Widgets {
	main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
	type Input = ();
	type Output = ();
	type Init = gtk::Window;
	type Root = ();
	type Widgets = Widgets;

	fn init_root() -> Self::Root {}

	fn init(
		main_window: Self::Init,
		_root: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {};

		let widgets = Widgets { main_window };

		ComponentParts { model, widgets }
	}

	fn update_view(
		&self,
		widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
		let dialog = adw::AboutWindow::builder()
			.icon_name(APP_ID)
			.application_icon(APP_ID)
			.application_name("Terms")
			.developer_name("Dirk Van Haerenborgh")
			.website("Website")
			.copyright("© 2022 Dirk Van Haerenborgh")
			.license_type(gtk::License::Mpl20)
			.website("https://github.com/vhdirk/terms/")
			.issue_url("https://github.com/vhdirk/terms/issues")
			.version(VERSION)
			.translator_credits(fl!("translator-credits").replace("\\n", "\n"))
			.modal(true)
			.transient_for(&widgets.main_window)
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