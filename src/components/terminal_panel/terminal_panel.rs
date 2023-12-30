use crate::components::{SearchToolbar, Terminal, TerminalInitArgs};
use adw::subclass::prelude::*;
use glib::ObjectExt;
use gtk::glib;
use gtk::CompositeTemplate;

use std::cell::RefCell;

use glib::{clone, subclass::Signal};
use once_cell::sync::Lazy;

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
