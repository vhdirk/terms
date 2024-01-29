use crate::components::Terminal;
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
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal_panel.ui")]
#[properties(wrapper_type = super::TerminalPanel)]
pub struct TerminalPanel {
    #[template_child]
    terminal: TemplateChild<Terminal>,

    #[property(get, set, construct, nullable)]
    directory: RefCell<Option<PathBuf>>,

    #[property(set, get, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(set, get, construct, nullable)]
    env: RefCell<Option<EnvMap>>,

    #[property(get, set, construct, nullable)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalPanel {
    const NAME: &'static str = "TermsTerminalPanel";
    type Type = super::TerminalPanel;
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
// // //           <lookup name="TerminalPanel">TerminalPanelTab</lookup>
// // //         </property> -->
// // //       </object>
// // //     </child>

// // //     <child>
// // //       <object class="TerminalPanelSearchToolbar" id="search_toolbar">
// // //         <!-- <binding name="TerminalPanel">
// // //           <lookup name="TerminalPanel">TerminalPanelTab</lookup>
// // //         </binding> -->

// // //         <property name="TerminalPanel" bind-source="TerminalPanelTab" bind-property="TerminalPanel" bind-flags="sync-create" />
// // //       </object>
// // //     </child>
// // //   </template>
