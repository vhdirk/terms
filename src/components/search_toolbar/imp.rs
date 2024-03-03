use adw::subclass::prelude::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/search_toolbar.ui")]
pub struct SearchToolbar {}

#[glib::object_subclass]
impl ObjectSubclass for SearchToolbar {
    const NAME: &'static str = "TermsSearchToolbar";
    type Type = super::SearchToolbar;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SearchToolbar {}
impl WidgetImpl for SearchToolbar {}

#[gtk::template_callbacks]
impl SearchToolbar {}
