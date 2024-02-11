use super::style_switcher_imp as imp;

glib::wrapper! {
        pub struct StyleSwitcher(ObjectSubclass<imp::StyleSwitcher>)
                @extends gtk::Widget, @implements gtk::Accessible;
}

impl StyleSwitcher {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
