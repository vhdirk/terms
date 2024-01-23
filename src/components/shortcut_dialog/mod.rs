mod shortcut_dialog;
use glib::{closure_local, subclass::prelude::*, IsA, ObjectExt};
use shortcut_dialog as imp;

glib::wrapper! {
        pub struct ShortcutDialog(ObjectSubclass<imp::ShortcutDialog>)
                @extends gtk::Widget, gtk::Window, adw::Window,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ShortcutDialog {
    pub fn new<W: IsA<gtk::Window>>(parent_window: Option<&W>, title: &str) -> Self {
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
