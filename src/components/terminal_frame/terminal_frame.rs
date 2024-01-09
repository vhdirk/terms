use crate::components::{SearchToolbar, Terminal, TerminalInitArgs};
use adw::subclass::prelude::*;
use glib::ObjectExt;
use gtk::glib;
use gtk::CompositeTemplate;
use tracing::*;

use std::cell::RefCell;

use glib::{clone, subclass::Signal};
use once_cell::sync::Lazy;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal_frame.ui")]
// #[properties(wrapper_type = super::TerminalFrame)]
pub struct TerminalFrame {
    pub init_args: RefCell<TerminalInitArgs>,

    #[template_child]
    terminal: TemplateChild<Terminal>,
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalFrame {
    const NAME: &'static str = "TermsTerminalFrame";
    type Type = super::TerminalFrame;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// #[glib::derived_properties]
impl ObjectImpl for TerminalFrame {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("exit").build()]);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for TerminalFrame {}
impl BoxImpl for TerminalFrame {}

#[gtk::template_callbacks]
impl TerminalFrame {
    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }

    fn setup_widgets(&self) {
        self.connect_signals();
    }

    fn connect_signals(&self) {
        self.terminal
            .connect_exit(clone!(@weak self as this => move |_terminal: &Terminal , status: i32| {
                                    info!("Terminal exited with {}", status);
                                    this.obj().emit_by_name::<()>("exit", &[]);
            }));
    }
}

// // //     <child>
// // //       <object class="GtkScrolledWindow" id="scrolled">
// // //         <!-- <property name="child">
// // //           <lookup name="TerminalFrame">TerminalFrameSession</lookup>
// // //         </property> -->
// // //       </object>
// // //     </child>

// // //     <child>
// // //       <object class="TerminalFrameSearchToolbar" id="search_toolbar">
// // //         <!-- <binding name="TerminalFrame">
// // //           <lookup name="TerminalFrame">TerminalFrameSession</lookup>
// // //         </binding> -->

// // //         <property name="TerminalFrame" bind-source="TerminalFrameSession" bind-property="TerminalFrame" bind-flags="sync-create" />
// // //       </object>
// // //     </child>
// // //   </template>

// //         },
