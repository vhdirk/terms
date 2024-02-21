use glib::{closure_local, prelude::*, subclass::prelude::*};

use super::{panel_imp as imp, utils::TwlWidgetExt, PanelHeader};

glib::wrapper! {
        pub struct Panel(ObjectSubclass<imp::Panel>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Panel {
    pub fn new(child: &impl IsA<gtk::Widget>, header: Option<&impl IsA<gtk::Widget>>) -> Self {
        let header = header.map(|h| h.as_ref().clone()).unwrap_or_else(|| PanelHeader::new().upcast());

        glib::Object::builder().property("child", child).property("header", header).build()
    }

    pub fn set_closing(&self, closing: bool) {
        self.imp().closing.set(closing);
    }

    pub fn closing(&self) -> bool {
        self.imp().closing.get()
    }

    pub fn connect_close_request<F: Fn(&Self) -> glib::Propagation + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "close-request",
            false,
            closure_local!(move |obj: Self| { f(&obj) == glib::Propagation::Proceed }),
        )
    }
}

impl TwlWidgetExt for Panel {}
