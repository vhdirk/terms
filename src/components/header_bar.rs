use crate::application::AppProfile;
use crate::config::PROFILE;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
	use super::*;

	#[derive(Debug, Default, gtk::CompositeTemplate)]
	#[template(resource = "/com/github/vhdirk/Terms/gtk/header_bar.ui")]
	pub struct HeaderBar {
		#[template_child]
		pub revealer: TemplateChild<gtk::Revealer>,

		#[template_child]
		pub title_widget: TemplateChild<adw::WindowTitle>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for HeaderBar {
		const NAME: &'static str = "TermsHeaderBar";
		type Type = super::HeaderBar;
		type ParentType = adw::Bin;

		fn class_init(klass: &mut Self::Class) {
			klass.bind_template();
		}

		fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
			obj.init_template();
		}
	}

	impl ObjectImpl for HeaderBar {}
	impl WidgetImpl for HeaderBar {}
	impl BinImpl for HeaderBar {}
}

glib::wrapper! {
		pub struct HeaderBar(ObjectSubclass<imp::HeaderBar>)
				@extends gtk::Widget, gtk::Window, gtk::HeaderBar,
				@implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl HeaderBar {
	pub fn new() -> Self {
		let obj: Self = glib::Object::builder().build();

		match PROFILE {
			AppProfile::Devel => {
				obj.set_css_classes(&["devel"]);
			},
			_ => (),
		}
		obj
	}
}
