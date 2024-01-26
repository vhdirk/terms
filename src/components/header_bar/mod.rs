mod header_bar;
use header_bar as imp;

glib::wrapper! {
        pub struct HeaderBar(ObjectSubclass<imp::HeaderBar>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl HeaderBar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj
    }
}
