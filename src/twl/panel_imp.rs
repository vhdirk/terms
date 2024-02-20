use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::basic::ClassStruct;
use glib::subclass::Signal;
use glib::Properties;
use gtk::{graphene, gsk};
use num_traits as num;
use once_cell::sync::Lazy;
use std::cell::{Cell, OnceCell, RefCell};
use std::cmp;
use std::marker::PhantomData;
use tracing::{info, warn};

use super::PanelHeader;

#[derive(Debug, Properties)]
#[properties(wrapper_type=super::Panel)]
pub struct Panel {
    #[property(get, set, construct)]
    needs_attention: Cell<bool>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<gio::Icon>>,

    #[property(get, set, construct, nullable)]
    tooltip: RefCell<Option<String>>,

    #[property(get, set=Self::set_content, construct, nullable)]
    content: RefCell<Option<gtk::Widget>>,

    #[property(get=Self::get_show_header, set=Self::set_show_header, construct)]
    show_header: PhantomData<bool>,

    pub closing: Cell<bool>,
    // #[property(get, set, construct)]
    // selected: Cell<bool>,

    // #[property(get, set, construct)]
    // live_thumbnail: Cell<bool>,
    #[property(get, explicit_notify)]
    header_height: Cell<i32>,

    #[property(get, set, builder(adw::ToolbarStyle::Flat))]
    header_style: Cell<adw::ToolbarStyle>,

    header_revealer: gtk::Revealer,

    #[property(get)]
    header: OnceCell<PanelHeader>,
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

impl Default for Panel {
    fn default() -> Self {
        Self {
            needs_attention: Default::default(),

            icon: Default::default(),

            tooltip: Default::default(),

            content: Default::default(),

            show_header: Default::default(),

            closing: Default::default(),

            header_height: Cell::new(-1),
            header_style: Cell::new(adw::ToolbarStyle::Flat),
            header_revealer: gtk::Revealer::new(),

            header: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Panel {
    const NAME: &'static str = "TwlPanel";
    type Type = super::Panel;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // klass.set_layout_manager_type::<gtk::BoxLayout>();
        klass.set_css_name("panel");
    }
}

#[glib::derived_properties]
impl ObjectImpl for Panel {
    fn constructed(&self) {
        self.parent_constructed();
        self.setup();
    }

    fn dispose(&self) {
        if let Some(content) = self.content.borrow().as_ref() {
            content.unparent();
        }

        self.header_revealer.set_child(None::<&gtk::Widget>);
        self.header_revealer.unparent();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("close").build()]);
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for Panel {
    fn request_mode(&self) -> gtk::SizeRequestMode {
        match self.content.borrow().as_ref() {
            Some(content) => content.request_mode(),
            None => gtk::SizeRequestMode::ConstantSize,
        }
    }

    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        let (header_min, header_nat, _, _) = self.header_revealer.measure(orientation, for_size);

        let (content_min, content_nat) = match self.content.borrow().as_ref() {
            Some(content) => {
                let (content_min, content_nat, _, _) = content.measure(orientation, for_size);
                (content_min, content_nat)
            },
            None => (0, 0),
        };

        let (minimum, natural) = match orientation {
            gtk::Orientation::Horizontal => (cmp::max(content_min, header_min), cmp::max(content_nat, header_nat)),
            _ => (content_min + header_min, content_nat + header_nat),
        };

        (minimum, natural, -1, -1)
    }

    fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
        let (header_min, header_nat, _, _) = self.header_revealer.measure(gtk::Orientation::Vertical, -1);

        let content_min = self
            .content
            .borrow()
            .as_ref()
            .map(|content| {
                let (content_min, _, _, _) = content.measure(gtk::Orientation::Vertical, -1);
                cmp::max(0, content_min)
            })
            .unwrap_or(0);

        let header_height = num::clamp(height - content_min, header_min, header_nat);
        let content_height = height - header_height;
        let content_offset = header_height;

        if self.header_height.get() != header_height {
            self.header_height.set(header_height);
            self.obj().notify_header_height();
        }

        self.header_revealer.allocate(width, header_height, -1, None);
        if let Some(content) = self.content.borrow().as_ref() {
            content.allocate(
                width,
                content_height,
                -1,
                Some(gsk::Transform::new().translate(&graphene::Point::new(0.0, content_offset as f32))),
            )
        }
    }
}

impl BuildableImpl for Panel {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        match (child.downcast_ref::<gtk::Widget>(), type_) {
            // (Some(widget), Some(wtype)) if wtype == "header" => match widget.clone().downcast::<PanelHeader>() {
            //     Err(err) => warn!("unable to use widget {:?} as header {:?}", widget, err),
            //     Ok(header) => self.set_header(Some(&header)),
            // },
            // (Some(widget), _) if wtype == "content" => self.set_content(Some(widget)),
            (Some(widget), _) => self.set_content(Some(widget)),
            (_, _) => self.parent_add_child(builder, child, type_),
        }
    }
}

impl Panel {
    fn setup(&self) {
        let header = PanelHeader::new(self.obj().as_ref());
        self.header.set(header.clone()).expect("Header should not have been set yet");
        self.obj().set_overflow(gtk::Overflow::Hidden);

        // self->header_style = ADW_TOOLBAR_FLAT;

        self.header_revealer.set_overflow(gtk::Overflow::Visible);
        self.header_revealer.set_vexpand(true);

        self.header_revealer.set_parent(&*self.obj());

        self.header_revealer.set_reveal_child(true);
        self.header_revealer.set_child(Some(&header));

        self.setup_content();
    }

    fn set_content(&self, content: Option<&gtk::Widget>) {
        info!("panel: set content {:?}", content);

        // TODO: not yet initialized? weird
        if self.header_revealer.parent().is_none() {
            *self.content.borrow_mut() = content.cloned();
            return;
        }

        if content == self.content.borrow().as_ref() {
            return;
        }

        if let Some(previous) = self.content.borrow().as_ref() {
            previous.unparent();
        }

        *self.content.borrow_mut() = content.cloned();
        self.setup_content();
    }

    fn setup_content(&self) {
        if let Some(content) = self.content.borrow().as_ref() {
            content.insert_before(&*self.obj(), Some(&self.header_revealer));
        }
    }
    fn set_show_header(&self, show_header: bool) {
        self.header_revealer.set_reveal_child(show_header)
    }

    fn get_show_header(&self) -> bool {
        self.header_revealer.is_child_revealed()
    }
}
