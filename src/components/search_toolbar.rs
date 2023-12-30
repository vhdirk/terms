use crate::application::AppProfile;
use crate::config::PROFILE;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Terms/gtk/search_toolbar.ui")]
    pub struct SearchToolbar {}

    #[glib::object_subclass]
    impl ObjectSubclass for SearchToolbar {
        const NAME: &'static str = "TermsSearchToolbar";
        type Type = super::SearchToolbar;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SearchToolbar {}
    impl WidgetImpl for SearchToolbar {}
}

glib::wrapper! {
        pub struct SearchToolbar(ObjectSubclass<imp::SearchToolbar>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchToolbar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj
    }
}
