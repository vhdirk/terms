use std::cell::RefCell;

use adw::prelude::*;
use adw::{prelude::BinExt, subclass::prelude::*};
use glib::{clone, subclass::Signal, Properties};
use once_cell::sync::Lazy;
use tracing::{info, warn};

use crate::twl::signal_accumulator_true_handled;

use super::{Paned, Panel};

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::PanelGrid)]
pub struct PanelGrid {
    pub inner: adw::Bin,

    #[property(get, nullable)]
    pub selected_panel: RefCell<Option<Panel>>,

    // TODO: perhaps we want to keep track of the location in the grid?
    pub panels: RefCell<Vec<Panel>>,
}

#[glib::object_subclass]
impl ObjectSubclass for PanelGrid {
    const NAME: &'static str = "TwlPanelGrid";
    type Type = super::PanelGrid;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

#[glib::derived_properties]
impl ObjectImpl for PanelGrid {
    fn constructed(&self) {
        self.parent_constructed();

        self.inner.set_parent(&*self.obj());
        self.connect_signals();
    }

    fn dispose(&self) {
        self.inner.unparent();
    }

    // Emitted after [PanelGrid.close_panel] has been called for @panel.
    //
    // The handler is expected to call [method@PanelGrid.close_panel_finish] to
    // confirm or reject the closing.
    //
    // The default handler will immediately confirm closing
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("close-panel")
                .param_types([Panel::static_type()])
                .run_last()
                .accumulator(signal_accumulator_true_handled)
                .return_type::<bool>()
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for PanelGrid {}

impl PanelGrid {
    fn connect_signals(&self) {
        // TODO: I haven't figured out yet how to install a class_handler in rust
        self.obj().connect_local(
            "close-panel",
            true,
            clone!(@weak self as this => @default-return None, move |values| {
                let panel = values[0].get::<Panel>().unwrap();
                this.close_panel_finish(&panel);
                Some((gdk::EVENT_STOP != 0).into())
            }),
        );
    }

    fn create_panel(&self, child: &impl IsA<gtk::Widget>) -> Panel {
        let panel = Panel::new(child);
        self.panels.borrow_mut().push(panel.clone());

        self.selected_panel.borrow_mut().replace(panel.clone());
        child.connect_has_focus_notify(clone!(@weak self as this, @weak panel as panel => move |_| {
            this.on_panel_focus(&panel);
        }));
        panel
    }

    pub fn set_child(&self, child: &impl IsA<gtk::Widget>) -> Panel {
        if let Some(orig_child) = self.inner.child() {
            orig_child.unparent();
        }

        let panel = self.create_panel(child);
        self.inner.set_child(Some(&panel));

        panel
    }

    pub fn split(&self, child: &impl IsA<gtk::Widget>, orientation: Option<gtk::Orientation>) -> Panel {
        let root_panel = self
            .selected_panel
            .borrow()
            .clone()
            .or_else(|| self.inner.child().and_then(|widget| self.panel(&widget)));
        info!("root panel {:?}", root_panel);

        let panel = self.create_panel(child);

        if let Some(root_panel) = root_panel {
            self.split_panel(&root_panel, &panel, orientation);
            panel
        } else {
            self.set_child(child)
        }
    }

    fn split_panel(&self, panel: &Panel, new_panel: &Panel, orientation: Option<gtk::Orientation>) {
        let new_paned = Paned::new(orientation.unwrap_or_else(|| self.preferred_orientation(panel)));

        match self.find_parent::<Paned>(panel) {
            // if the widget does not belong to a paned, it has to be the root
            None => {
                info!("setting root child {:?}", new_paned);
                panel.unparent();
                self.inner.set_child(Some(&new_paned));
            },
            Some(existing_paned) => {
                existing_paned.replace(Some(panel), Some(&new_paned));
            },
        }

        new_paned.set_start_child(Some(panel.clone()));
        new_paned.set_end_child(Some(new_panel.clone()));
    }

    fn preferred_orientation(&self, panel: &Panel) -> gtk::Orientation {
        if panel.width() > panel.height() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        }
    }

    fn find_parent<T: IsA<gtk::Widget>>(&self, widget: &impl IsA<gtk::Widget>) -> Option<T> {
        let mut widget = Some(Into::<gtk::Widget>::into(widget.clone()));

        while let Some(current) = widget {
            if current.is::<T>() {
                return current.downcast().ok();
            }
            if current.is::<super::PanelGrid>() {
                return None;
            }
            widget = current.parent();
        }
        None
    }

    pub fn on_panel_focus(&self, panel: &Panel) {
        self.selected_panel.borrow_mut().replace(panel.clone());
        self.obj().notify_selected_panel();
    }

    pub fn close_other_panels(&self, panel: &Panel) {
        todo!();
    }

    pub fn close_panel(&self, panel: &Panel) {
        if panel.closing() {
            return;
        }

        panel.set_closing(true);

        self.obj().emit_by_name::<bool>("close-panel", &[panel]);
    }

    pub fn close_panel_finish(&self, panel: &Panel) {
        if !panel.closing() {
            warn!("Will not finish closing a panel that was not in closing state");
            return;
        }

        match self.find_parent::<Paned>(panel) {
            // if the widget does not belong to a paned, it has to be the root
            None => self.inner.set_child(None::<&gtk::Widget>),
            Some(paned) => {
                paned.unparent();
                let sibling = paned.sibling(Some(panel));
                if let Some(sibling) = sibling.as_ref() {
                    sibling.unparent();
                }

                match self.find_parent::<Paned>(&paned) {
                    Some(parent_paned) => {
                        parent_paned.replace(Some(&paned), sibling.as_ref());
                    },
                    None => {
                        self.inner.set_child(sibling.as_ref());
                    },
                }
            },
        }
    }

    pub fn panel(&self, widget: &impl IsA<gtk::Widget>) -> Option<Panel> {
        self.find_parent::<Panel>(widget)
    }
}
