use crate::components::StyleSwitcher;
use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/header_bar.ui")]
pub struct HeaderBar {
    // #[template_child]
    // pub revealer: TemplateChild<gtk::Revealer>,
    #[template_child]
    pub title_widget: TemplateChild<adw::WindowTitle>,

    #[template_child]
    pub style_switcher: TemplateChild<StyleSwitcher>,
}

#[glib::object_subclass]
impl ObjectSubclass for HeaderBar {
    const NAME: &'static str = "TermsHeaderBar";
    type Type = super::HeaderBar;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("headerbar");
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for HeaderBar {}
impl WidgetImpl for HeaderBar {}
impl BinImpl for HeaderBar {}
