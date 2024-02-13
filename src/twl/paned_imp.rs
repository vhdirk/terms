use std::marker::PhantomData;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use tracing::*;

#[derive(Debug, Clone)]
enum ChildPosition {
    Start,
    End
}

impl ToString for ChildPosition {
    fn to_string(&self) -> String {
        match self {
            Self::Start => "start".to_string(),
            Self::End => "end".to_string()
        }
    }
}

#[derive(Debug, Properties)]
#[properties(wrapper_type=super::Paned)]
pub struct Paned {
    inner: gtk::Paned,

    #[property(get=Self::get_position, set=Self::set_position, construct, default=-1)]
    position: PhantomData<i32>,

    #[property(get=Self::is_position_set)]
    position_set: PhantomData<bool>,

    #[property(get=Self::get_min_position)]
    min_position: PhantomData<i32>,

    #[property(get=Self::get_max_position)]
    max_position: PhantomData<i32>,

    #[property(get=Self::is_wide_handle, set=Self::set_wide_handle, construct)]
    wide_handle: PhantomData<bool>,

    #[property(get=Self::get_resize_start_child, set=Self::set_resize_start_child, construct)]
    resize_start_child: PhantomData<bool>,

    #[property(get=Self::get_resize_end_child, set=Self::set_resize_end_child, construct)]
    resize_end_child: PhantomData<bool>,

    #[property(get=Self::get_shrink_start_child, set=Self::set_shrink_start_child, construct)]
    shrink_start_child: PhantomData<bool>,

    #[property(get=Self::get_shrink_end_child, set=Self::set_shrink_end_child, construct)]
    shrink_end_child: PhantomData<bool>,

    #[property(get=Self::get_orientation, set=Self::set_orientation, construct, builder(gtk::Orientation::Horizontal))]
    orientation: PhantomData<gtk::Orientation>,

    #[property(get=Self::get_start_child, set=Self::set_start_child, construct, nullable)]
    start_child: PhantomData<Option<gtk::Widget>>,

    #[property(get=Self::get_end_child, set=Self::set_end_child, construct, nullable)]
    end_child: PhantomData<Option<gtk::Widget>>,
}

impl Default for Paned {
    fn default() -> Self {
        Self {
            inner: gtk::Paned::new(gtk::Orientation::Horizontal),
            position: Default::default(),
            position_set: Default::default(),
            min_position: Default::default(),
            max_position: Default::default(),

            wide_handle: Default::default(),
            resize_start_child: Default::default(),
            resize_end_child: Default::default(),
            shrink_start_child: Default::default(),
            shrink_end_child: Default::default(),

            orientation: Default::default(),
            start_child: Default::default(),
            end_child: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Paned {
    const NAME: &'static str = "TwlPaned";
    type Type = super::Paned;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Buildable, gtk::Orientable);

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Paned {
    fn constructed(&self) {
        self.parent_constructed();

        self.inner.set_parent(&*self.obj());
        self.obj().set_focusable(true);
        self.obj().set_focus_child(Some(&self.inner));
        self.get_or_init_container(ChildPosition::Start);
        self.get_or_init_container(ChildPosition::End);
    }

    fn dispose(&self) {
        self.inner.unparent();
    }
}
impl WidgetImpl for Paned {}

impl OrientableImpl for Paned {}

impl BuildableImpl for Paned {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        match child.downcast_ref::<gtk::Widget>() {
            Some(widget) => match type_ {
                Some(position) if position == "start" => {
                    self.set_start_child(Some(widget));
                    self.set_resize_start_child(true);
                    self.set_shrink_start_child(false);
                },
                Some(position) if position == "end" => {
                    self.set_end_child(Some(widget));
                    self.set_resize_end_child(true);
                    self.set_shrink_end_child(false);
                },
                None if self.get_start_child().is_none() => {
                    self.set_start_child(Some(widget));
                    self.set_resize_start_child(true);
                    self.set_shrink_start_child(false);
                },
                None if self.get_end_child().is_none() => {
                    self.set_end_child(Some(widget));
                    self.set_resize_end_child(true);
                    self.set_shrink_end_child(false);
                },
                _ => warn!("TwlPaned only accepts two widgets as children"),
            },
            None => self.parent_add_child(builder, child, type_),
        };
    }
}

impl Paned {
    fn get_position(&self) -> i32 {
        self.inner.position()
    }

    fn set_position(&self, position: i32) {
        self.inner.set_position(position)
    }

    fn is_position_set(&self) -> bool {
        self.inner.is_position_set()
    }

    fn get_min_position(&self) -> i32 {
        self.inner.min_position()
    }

    fn get_max_position(&self) -> i32 {
        self.inner.max_position()
    }

    fn is_wide_handle(&self) -> bool {
        self.inner.is_wide_handle()
    }

    fn set_wide_handle(&self, wide_handle: bool) {
        self.inner.set_wide_handle(wide_handle)
    }

    fn get_resize_start_child(&self) -> bool {
        self.inner.resizes_start_child()
    }

    fn set_resize_start_child(&self, resize_start_child: bool) {
        self.inner.set_resize_start_child(resize_start_child)
    }

    fn get_resize_end_child(&self) -> bool {
        self.inner.resizes_end_child()
    }

    fn set_resize_end_child(&self, resize_end_child: bool) {
        self.inner.set_resize_end_child(resize_end_child)
    }

    fn get_shrink_start_child(&self) -> bool {
        self.inner.shrinks_start_child()
    }

    fn set_shrink_start_child(&self, shrink_start_child: bool) {
        self.inner.set_shrink_start_child(shrink_start_child)
    }

    fn get_shrink_end_child(&self) -> bool {
        self.inner.shrinks_end_child()
    }

    fn set_shrink_end_child(&self, shrink_end_child: bool) {
        self.inner.set_shrink_end_child(shrink_end_child)
    }

    fn get_orientation(&self) -> gtk::Orientation {
        self.inner.orientation()
    }

    fn set_orientation(&self, orientation: gtk::Orientation) {
        self.inner.set_orientation(orientation)
    }

    fn get_or_init_container(&self, position: ChildPosition) -> adw::Bin {
        let child = match position {
            ChildPosition::Start => self.inner.start_child(),
            ChildPosition::End => self.inner.end_child()
        };
        child.and_downcast::<adw::Bin>().unwrap_or_else(|| {
            let bin = adw::Bin::new();
            // bin.set_hexpand(true);
            // bin.set_vexpand(true);

            let resize = true;
            let shrink = false;

            match position {
                ChildPosition::Start => {
                    self.inner.set_start_child(Some(&bin));
                    // self.set_resize_start_child(resize);
                    // self.set_shrink_start_child(shrink);
                },
                ChildPosition::End => {
                    self.inner.set_end_child(Some(&bin));
                    // self.set_resize_end_child(resize);
                    // self.set_shrink_end_child(shrink);
                },
            }

            bin
        })
    }

    fn get_start_child(&self) -> Option<gtk::Widget> {
        self.get_or_init_container(ChildPosition::Start).child()
    }

    fn set_start_child(&self, child: Option<&gtk::Widget>) {
        debug!("Paned::set_start_child {:?}", child);
        let bin = self.get_or_init_container(ChildPosition::Start);
        // if let Some(c) = child {
        //     if c.parent().is_some() {
        //         c.unparent();
        //     }
        // }
        bin.set_child(child);
    }

    fn get_end_child(&self) -> Option<gtk::Widget> {
        self.get_or_init_container(ChildPosition::End).child()
    }

    fn set_end_child(&self, child: Option<&gtk::Widget>) {
        debug!("Paned::set_end_child {:?}", child);

        debug!("Paned: end child {:?}", self.inner.end_child());
        let bin = self.get_or_init_container(ChildPosition::End);
        // if let Some(c) = child {
        //     if c.parent().is_some() {
        //         c.unparent();
        //     }
        // }
        bin.set_child(child);
    }

    pub fn replace(&self, child: Option<gtk::Widget>, new_child: Option<gtk::Widget>) {
        debug!("Panel::replace {:?} with {:?}", child, new_child);
        debug!("Panel::start_child {:?}", self.get_start_child());
        debug!("Panel::end_child {:?}", self.get_end_child());

        if self.get_start_child() == child {
            if let Some(child) = child {
                child.unparent();
            }
            self.set_start_child(new_child.as_ref());
        } else if self.get_end_child() == child {
            if let Some(child) = child {
                child.unparent();
            }
            self.set_end_child(new_child.as_ref());
        } else {
            warn!("Not a parent of child {:?}", child);
        }
    }

    pub fn sibling(&self, child: Option<gtk::Widget>) -> Option<gtk::Widget> {
        debug!("Panel::get_sibling {:?}", child);

        if self.get_start_child() == child {
            self.get_end_child()
        } else if self.get_end_child() == child {
            self.get_start_child()
        } else {
            warn!("Not a parent of child {:?}", child);
            None
        }
    }
}
