use std::path::PathBuf;

use adw::subclass::prelude::*;
use glib::closure_local;
use glib::value::ValueType;
use glib::ObjectExt;
use glib::Value;
use gtk::glib;
use gtk::CompositeTemplate;

use super::terminal::TerminalInitArgs;

mod imp {
    use std::cell::RefCell;

    use glib::{clone, subclass::Signal};
    use once_cell::sync::Lazy;

    use crate::components::terminal_panel::TerminalPanel;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Terms/gtk/session.ui")]
    // #[properties(wrapper_type = super::Session)]
    pub struct Session {
        pub init_args: RefCell<TerminalInitArgs>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Session {
        const NAME: &'static str = "TermsSession";
        type Type = super::Session;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // #[glib::derived_properties]
    impl ObjectImpl for Session {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup_widgets();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("close").build()]);
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for Session {}
    impl BinImpl for Session {}

    #[gtk::template_callbacks]
    impl Session {
        pub fn set_init_args(&self, init_args: TerminalInitArgs) {
            let mut args = self.init_args.borrow_mut();
            *args = init_args;
        }

        fn setup_widgets(&self) {
            let panel = TerminalPanel::new(self.init_args.borrow().clone());
            self.obj().set_property("child", &panel);

            panel.connect_exit(clone!(@weak self as this => move |panel| {
                                this.obj().emit_by_name::<()>("close", &[]);
            }));
        }
    }
}

glib::wrapper! {
        pub struct Session(ObjectSubclass<imp::Session>)
                @extends gtk::Widget, adw::Bin,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
    }

    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "close",
            true,
            closure_local!(move |obj: Session| {
                f(&obj);
            }),
        )
    }
}
