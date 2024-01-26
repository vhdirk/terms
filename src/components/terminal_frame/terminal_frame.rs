use crate::components::{SearchToolbar, Terminal, TerminalInitArgs};
use crate::util::EnvMap;
use adw::subclass::prelude::*;
use glib::ObjectExt;
use glib::Properties;
use gtk::glib;
use gtk::CompositeTemplate;
use tracing::*;

use std::cell::RefCell;
use std::path::PathBuf;

use glib::{clone, subclass::Signal};
use once_cell::sync::Lazy;

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal_frame.ui")]
#[properties(wrapper_type = super::TerminalFrame)]
pub struct TerminalFrame {
    #[template_child]
    terminal: TemplateChild<Terminal>,

    #[property(get, set, construct, nullable)]
    working_directory: RefCell<Option<PathBuf>>,

    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(set, get, construct, nullable)]
    env: RefCell<Option<EnvMap>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalFrame {
    const NAME: &'static str = "TermsTerminalFrame";
    type Type = super::TerminalFrame;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
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
