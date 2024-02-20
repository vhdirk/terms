use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::marker::PhantomData;

use adw::prelude::*;
use adw::{prelude::BinExt, subclass::prelude::*};
use glib::{clone, subclass::Signal, Properties};
use once_cell::sync::Lazy;
use tracing::*;

use crate::twl::utils::signal_accumulator_propagation;

use super::split_paned::SplitPaned;
use super::utils::TwlWidgetExt;
use super::Panel;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::PanelGrid)]
pub struct PanelGrid {
    pub inner: adw::Bin,

    #[property(get, nullable)]
    pub selected_panel: RefCell<Option<Panel>>,

    #[property(get, set=Self::set_wide_handle, construct)]
    wide_handle: Cell<bool>,

    #[property(get, set=Self::set_show_panel_headers, construct)]
    show_panel_headers: Cell<bool>,

    #[property(get=Self::get_n_panels)]
    n_panels: PhantomData<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for PanelGrid {
    const NAME: &'static str = "TwlPanelGrid";
    type Type = super::PanelGrid;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.set_css_name("panel_grid");
    }
}

#[glib::derived_properties]
impl ObjectImpl for PanelGrid {
    fn constructed(&self) {
        self.parent_constructed();

        self.inner.set_parent(&*self.obj());
        self.connect_signals();

        // paned.connect_cycle_child_focus(f)
    }

    fn dispose(&self) {
        self.inner.unparent();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                // Emitted after [PanelGrid.close_panel] has been called for @panel.
                //
                // The handler is expected to call [method@PanelGrid.close_panel_finish] to
                // confirm or reject the closing.
                //
                // The default handler will immediately confirm closing
                Signal::builder("close-panel")
                    .param_types([Panel::static_type()])
                    .run_last()
                    .action()
                    .accumulator(signal_accumulator_propagation)
                    .return_type::<bool>()
                    .class_handler(|_, args| {
                        debug!("close-panel class_handler");
                        let this = args[0].get::<super::PanelGrid>().expect("signal arg");
                        let panel = args[1].get::<Panel>().expect("signal arg");

                        this.close_panel_finish(&panel);

                        Some(Into::<bool>::into(glib::Propagation::Stop).into())
                    })
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for PanelGrid {}

impl PanelGrid {
    fn connect_signals(&self) {}

    fn set_wide_handle(&self, wide_handle: bool) {
        self.wide_handle.set(wide_handle);

        for paned in self.get_all::<gtk::Paned>().iter() {
            // TODO: can we filter for Paned only created by us?
            paned.set_wide_handle(wide_handle);
        }
    }

    fn set_show_panel_headers(&self, show_panel_headers: bool) {
        self.show_panel_headers.set(show_panel_headers);

        self.update_headers_visibility();
    }

    fn update_headers_visibility(&self) {
        let panels = self.get_all::<Panel>();

        if panels.len() == 1 {
            panels[0].set_show_header(false);
        } else {
            for panel in panels.iter() {
                panel.set_show_header(self.show_panel_headers.get());
            }
        }
    }

    pub fn get_all<T>(&self) -> Vec<T>
    where
        T: IsA<gtk::Widget> + ObjectType,
    {
        self.get_all_inner(&self.inner.upcast_ref()).into_iter().collect()
    }

    fn get_all_inner<T>(&self, root: &gtk::Widget) -> HashSet<T>
    where
        T: IsA<gtk::Widget> + ObjectType,
    {
        let mut elems = HashSet::new();

        if let Ok(relem) = root.clone().downcast() {
            elems.insert(relem);
        }

        for child in root.iter_children() {
            if let Ok(elem) = child.clone().downcast() {
                elems.insert(elem);
            }

            let child_elems = self.get_all_inner(&child);
            elems.extend(child_elems);
        }

        elems
    }

    fn create_panel(&self, child: &impl IsA<gtk::Widget>) -> Panel {
        let panel = Panel::new(child);

        self.selected_panel.borrow_mut().replace(panel.clone());
        child.connect_has_focus_notify(clone!(@weak self as this, @weak panel as panel => move |c| {
            if c.has_focus() {
                this.on_panel_focus(&panel);
            }
        }));

        panel.connect_close(clone!(@weak self as this => move |p| {
            this.close_panel(p);
        }));

        panel
    }

    pub fn set_initial_child(&self, child: &impl IsA<gtk::Widget>) -> Panel {
        let panel = self.create_panel(child);
        self.inner.set_child(Some(&panel));

        self.update_headers_visibility();
        self.obj().notify_n_panels();

        panel
    }

    pub fn split(&self, child: &impl IsA<gtk::Widget>, orientation: Option<gtk::Orientation>) -> Panel {
        let active_panel = self.selected_panel.borrow().clone().or_else(|| self.get_all::<Panel>().first().cloned());
        debug!("active panel {:?}", active_panel);

        if let Some(active_panel) = active_panel {
            debug!("split: split active panel");
            let panel = self.create_panel(child);
            self.split_panel(&active_panel, &panel, orientation);
            panel
        } else {
            debug!("split: set initial child");
            self.set_initial_child(child)
        }
    }

    fn split_panel(&self, panel: &Panel, new_panel: &Panel, orientation: Option<gtk::Orientation>) {
        let new_paned = gtk::Paned::new(orientation.unwrap_or_else(|| self.preferred_orientation(panel)));
        new_paned.set_wide_handle(self.wide_handle.get());
        debug!("panel {:?} parent {:?}", panel, panel.parent());

        match panel.parent().and_downcast::<gtk::Paned>() {
            // if the widget does not belong to a paned, it has to be the root
            None => {
                debug!("setting root child {:?}", new_paned);
                self.inner.set_child(Some(&new_paned));
            },
            Some(parent_paned) => {
                parent_paned.replace(Some(panel), Some(&new_paned));
            },
        }

        new_paned.set_start_child(Some(panel));
        new_paned.set_end_child(Some(new_panel));
        self.update_headers_visibility();
        self.obj().notify_n_panels();
    }

    fn preferred_orientation(&self, panel: &Panel) -> gtk::Orientation {
        if panel.width() > panel.height() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        }
    }

    fn find_parent<T: IsA<gtk::Widget>>(&self, widget: &impl IsA<gtk::Widget>) -> Option<T> {
        let mut widget = Some(widget.as_ref().clone());

        while let Some(current) = widget {
            if current.is::<T>() {
                return current.downcast().ok();
            }
            if current == *self.obj() {
                return None;
            }
            widget = current.parent();
        }
        None
    }

    pub fn on_panel_focus(&self, panel: &Panel) {
        debug!("On panel focus {:?}", panel);
        self.selected_panel.set(Some(panel.clone()));
        self.obj().notify_selected_panel();
    }

    pub fn close_other_panels(&self, panel: &Panel) {
        let panels = self.get_all::<Panel>();
        for other_panel in panels.iter() {
            if other_panel != panel {
                self.close_panel(other_panel);
            }
        }
    }

    pub fn close_panel(&self, panel: &Panel) {
        debug!("request to close panel: {:?}", panel);
        if panel.closing() {
            warn!("Panel {:?} is already closing", panel);
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
        debug!("Closing panel: {:?}", panel);
        match panel.parent().and_downcast::<gtk::Paned>() {
            // if the widget does not belong to a paned, it has to be the root
            None => {
                debug!("No parent paned found");
                self.inner.set_child(None::<&gtk::Widget>);
            },
            Some(paned) => {
                let sibling = paned.sibling(Some(panel));
                debug!("got sibling {:?}", sibling);

                paned.set_start_child(None::<&gtk::Widget>);
                paned.set_end_child(None::<&gtk::Widget>);

                match paned.parent().and_downcast::<gtk::Paned>() {
                    Some(parent_paned) => {
                        parent_paned.replace(Some(&paned), sibling.as_ref());
                    },
                    None => {
                        debug!("Setting sibling {:?} as root child", sibling);
                        self.inner.set_child(sibling.as_ref());
                    },
                }
            },
        }

        // TODO: disconnect signals
        self.update_headers_visibility();
        self.obj().notify_n_panels();
    }

    pub fn panel(&self, widget: &impl IsA<gtk::Widget>) -> Option<Panel> {
        self.find_parent::<Panel>(widget)
    }

    pub fn get_n_panels(&self) -> u32 {
        self.get_all::<Panel>().len() as u32
    }
}
