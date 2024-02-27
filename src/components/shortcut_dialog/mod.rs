mod imp;
use glib::{closure_local, prelude::*, subclass::prelude::*};

glib::wrapper! {
        pub struct ShortcutDialog(ObjectSubclass<imp::ShortcutDialog>)
                @extends gtk::Widget, gtk::Window, adw::Window,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ShortcutDialog {
    pub fn new(parent_window: Option<&impl IsA<gtk::Window>>, title: &str) -> Self {
        glib::Object::builder()
            .property("transient-for", parent_window)
            .property("shortcut", title)
            .build()
    }

    pub fn connect_response<F: Fn(&Self, gtk::ResponseType, Option<String>) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "response",
            true,
            closure_local!(move |obj: ShortcutDialog, response_type: gtk::ResponseType, accel: Option<String>| {
                f(&obj, response_type, accel);
            }),
        )
    }
}
