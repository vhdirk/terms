use std::cell::{Cell, RefCell};

use adw::subclass::prelude::*;
use glib::prelude::*;
use glib::subclass::Signal;
use glib::Properties;
use once_cell::sync::Lazy;
use tracing::warn;
use vte::BoxExt;
use vte::WidgetExt;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::Panel)]
pub struct Panel {
    container: gtk::Box,

    #[property(get, set=Self::set_title, construct, nullable)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct)]
    needs_attention: Cell<bool>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<gio::Icon>>,

    #[property(get, set, construct, nullable)]
    tooltip: RefCell<Option<String>>,

    #[property(get, set=Self::set_child, construct, nullable)]
    child: RefCell<Option<gtk::Widget>>,

    #[property(get, set=Self::set_title_widget, construct, nullable)]
    title_widget: RefCell<Option<gtk::Widget>>,

    #[property(get, set=Self::set_show_header, construct)]
    show_header: Cell<bool>,

    pub closing: Cell<bool>,
    // #[property(get, set, construct)]
    // selected: Cell<bool>,

    // #[property(get, set, construct)]
    // live_thumbnail: Cell<bool>,

    //   PAGE_PROP_PARENT,
    //   PAGE_PROP_SELECTED,
    //   PAGE_PROP_LOADING,
    //   PAGE_PROP_INDICATOR_ICON,
    //   PAGE_PROP_INDICATOR_TOOLTIP,
    //   PAGE_PROP_INDICATOR_ACTIVATABLE,
    //   PAGE_PROP_KEYWORD,
    //   PAGE_PROP_THUMBNAIL_XALIGN,
    //   PAGE_PROP_THUMBNAIL_YALIGN,
    //   PAGE_PROP_LIVE_THUMBNAIL,
}

#[glib::object_subclass]
impl ObjectSubclass for Panel {
    const NAME: &'static str = "TwlPanel";
    type Type = super::Panel;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Panel {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup();
    }

    fn dispose(&self) {
        self.container.unparent();
        if let Some(title_widget) = self.title_widget.borrow().as_ref() {
            title_widget.unparent();
        }
    }
}
impl WidgetImpl for Panel {}

impl BuildableImpl for Panel {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        match (child.downcast_ref::<gtk::Widget>(), type_) {
            (Some(widget), Some(wtype)) if wtype == "title" => self.set_title_widget(Some(widget)),
            (Some(widget), _) => self.set_child(Some(widget)),
            (None, _) => self.parent_add_child(builder, child, type_),
        }
    }
}

impl Panel {
    fn setup(&self) {
        self.container.set_parent(&*self.obj());
        self.obj().set_focusable(true);
        self.obj().set_focus_child(Some(&self.container));
    }

    fn set_child(&self, child: Option<&gtk::Widget>) {
        self.remove_child();

        if let Some(child) = child {
            self.container.append(child);
            *self.child.borrow_mut() = Some(child.clone());
        }

        self.container.set_focus_child(child);
    }

    fn set_title_widget(&self, child: Option<&gtk::Widget>) {
        if self.title_widget.borrow().as_ref() == child {
            return;
        }

        if let Some(widget) = self.title_widget.borrow().as_ref() {
            widget.unparent();
        }

        *self.title_widget.borrow_mut() = child.cloned();

        if let Some(widget) = child {
            widget.set_parent(&*self.obj());
        }

        let _ = self.obj().freeze_notify();

        self.obj().notify_title_widget();
        self.obj().notify_title();
    }

    fn set_title(&self, title: Option<&str>) {
        // match title {
        //     Some(title) => {
        //         self.set_title_widget(Some(gtk::Label::new(Some(title)).upcast_ref()));
        //     },
        //     None => {
        //         self.set_title_widget(None::<&gtk::Widget>);
        //     },
        // }
    }

    fn remove_child(&self) {
        if let Some(child) = self.child.borrow().as_ref() {
            child.unparent();
        }
        *self.child.borrow_mut() = None;
    }

    fn set_show_header(&self, show_header: bool) {
        self.show_header.set(show_header);
        // TODO
    }
}
