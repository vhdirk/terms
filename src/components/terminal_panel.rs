use crate::components::{SearchToolbar, Terminal, TerminalInitArgs};
use adw::subclass::prelude::*;
use glib::value::ValueType;
use glib::{closure_local, ObjectExt, Value};
use gtk::glib;
use gtk::CompositeTemplate;

mod imp {
    use std::cell::{Cell, RefCell};

    use glib::{clone, closure_local, subclass::Signal, ObjectExt};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Terms/gtk/terminal_panel.ui")]
    // #[properties(wrapper_type = super::TerminalPanel)]
    pub struct TerminalPanel {
        pub init_args: RefCell<TerminalInitArgs>,

        #[template_child]
        terminal: TemplateChild<Terminal>,

        #[template_child]
        search_toolbar: TemplateChild<SearchToolbar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TerminalPanel {
        const NAME: &'static str = "TermsTerminalPanel";
        type Type = super::TerminalPanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // #[glib::derived_properties]
    impl ObjectImpl for TerminalPanel {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup_widgets();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("exit").build()]);
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for TerminalPanel {}
    impl BoxImpl for TerminalPanel {}

    #[gtk::template_callbacks]
    impl TerminalPanel {
        pub fn set_init_args(&self, init_args: TerminalInitArgs) {
            let mut args = self.init_args.borrow_mut();
            *args = init_args;
        }

        fn setup_widgets(&self) {
            self.connect_signals();
        }

        fn connect_signals(&self) {
            dbg!("Connect exit closure");
            self.terminal
                .connect_exit(clone!(@weak self as this => move |_terminal: &Terminal , status: i32| {
                                        println!("Terminal exited with {}", status);
                                        this.obj().emit_by_name::<()>("exit", &[]);
                }));
        }
    }
}

glib::wrapper! {
        pub struct TerminalPanel(ObjectSubclass<imp::TerminalPanel>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TerminalPanel {
    pub fn new(init_args: TerminalInitArgs) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().set_init_args(init_args);
        obj
    }

    pub fn connect_exit<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "exit",
            true,
            closure_local!(move |obj: TerminalPanel| {
                f(&obj);
            }),
        )
    }
}

// // //     <child>
// // //       <object class="GtkScrolledWindow" id="scrolled">
// // //         <!-- <property name="child">
// // //           <lookup name="TerminalPanel">TerminalPanelSession</lookup>
// // //         </property> -->
// // //       </object>
// // //     </child>

// // //     <child>
// // //       <object class="TerminalPanelSearchToolbar" id="search_toolbar">
// // //         <!-- <binding name="TerminalPanel">
// // //           <lookup name="TerminalPanel">TerminalPanelSession</lookup>
// // //         </binding> -->

// // //         <property name="TerminalPanel" bind-source="TerminalPanelSession" bind-property="TerminalPanel" bind-flags="sync-create" />
// // //       </object>
// // //     </child>
// // //   </template>

// //         },
