use std::{cell::Cell, marker::PhantomData};

use adw::{prelude::*, subclass::prelude::*};
use glib::Properties;
use gtk::{graphene, gsk};

use approx::abs_diff_eq;
use num_traits as num;

const DEFAULT_FADE_WIDTH: f32 = 18.0;

#[derive(Debug, Properties)]
#[properties(wrapper_type=super::FadingLabel)]
pub struct FadingLabel {
    #[property(get=Self::get_label, set=Self::set_label, construct, explicit_notify)]
    label: PhantomData<String>,

    #[property(get, set=Self::set_align, minimum=0.0, maximum=1.0, default=0.0, construct, explicit_notify)]
    align: Cell<f32>,

    #[property(get, set=Self::set_fade_width, default=DEFAULT_FADE_WIDTH, construct, explicit_notify)]
    fade_width: Cell<f32>,

    label_widget: gtk::Label,
}

impl Default for FadingLabel {
    fn default() -> Self {
        Self {
            label_widget: gtk::Label::new(None),
            label: Default::default(),
            align: Cell::new(0.0),
            fade_width: Cell::new(DEFAULT_FADE_WIDTH),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FadingLabel {
    const NAME: &'static str = "TwlFadingLabel";
    type Type = super::FadingLabel;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for FadingLabel {
    fn constructed(&self) {
        self.parent_constructed();

        self.label_widget.set_parent(&*self.obj());
        self.label_widget.set_single_line_mode(true);
    }

    fn dispose(&self) {
        self.label_widget.unparent();
    }
}

impl WidgetImpl for FadingLabel {
    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        let (mut min, nat, min_baseline, nat_baseline) = self.label_widget.measure(orientation, for_size);

        if orientation == gtk::Orientation::Horizontal && min > 0 {
            min = 0;
        }
        (min, nat, min_baseline, nat_baseline)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let align = if self.is_rtl() { 1.0 - self.align.get() } else { self.align.get() };

        let (_, child_width, _, _) = self.label_widget.measure(gtk::Orientation::Horizontal, height);

        let offset = (width as f32 - child_width as f32) * align;
        let transform = gsk::Transform::new().translate(&graphene::Point::new(offset, 0.0));

        self.label_widget.allocate(child_width, height, baseline, Some(transform));
    }

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let align = if self.is_rtl() { 1.0 - self.align.get() } else { self.align.get() };
        let width = self.obj().width();

        if width <= 0 {
            return;
        }

        let clipped_size = self.label_widget.width() - width;

        if clipped_size <= 0 {
            self.obj().snapshot_child(&self.label_widget, snapshot);
            return;
        }

        let width = width as f32;
        let child_snapshot = gtk::Snapshot::new();
        self.obj().snapshot_child(&self.label_widget, &child_snapshot);

        let node = child_snapshot.to_node();

        if node.is_none() {
            self.obj().snapshot_child(&self.label_widget, snapshot);
            return;
        }

        let node = node.unwrap();

        let node_bounds = node.bounds();
        let bounds = graphene::Rect::new(0.0, node_bounds.y().floor(), width, f32::ceil(node_bounds.height() + 1.0));

        snapshot.push_mask(gsk::MaskMode::InvertedAlpha);

        if align > 0.0 {
            snapshot.append_linear_gradient(
                &graphene::Rect::new(0.0, bounds.y(), self.fade_width.get(), bounds.height()),
                &graphene::Point::new(0.0, 0.0),
                &graphene::Point::new(self.fade_width.get(), 0.0),
                &[
                    gsk::ColorStop::new(0.0, gdk::RGBA::new(0.0, 0.0, 0.0, 1.0)),
                    gsk::ColorStop::new(1.0, gdk::RGBA::new(0.0, 0.0, 0.0, 0.0)),
                ],
            );
        }

        if align < 1.0 {
            snapshot.append_linear_gradient(
                &graphene::Rect::new(width - self.fade_width.get(), bounds.y(), self.fade_width.get(), bounds.height()),
                &graphene::Point::new(width, 0.0),
                &graphene::Point::new(width - self.fade_width.get(), 0.0),
                &[
                    gsk::ColorStop::new(0.0, gdk::RGBA::new(0.0, 0.0, 0.0, 1.0)),
                    gsk::ColorStop::new(1.0, gdk::RGBA::new(0.0, 0.0, 0.0, 0.0)),
                ],
            );
        }

        snapshot.pop();

        snapshot.push_clip(&bounds);
        snapshot.append_node(&node);
        snapshot.pop();

        snapshot.pop();
    }
}

impl FadingLabel {
    fn set_label(&self, label: &str) {
        if label == self.get_label() {
            return;
        }

        self.label_widget.set_label(label);
        self.obj().notify_label();
    }

    fn get_label(&self) -> String {
        self.label_widget.label().into()
    }

    fn set_align(&self, align: f32) {
        let align = num::clamp(align, 0.0, 1.0);

        if abs_diff_eq!(self.align.get(), align) {
            return;
        }

        self.align.set(align);

        self.obj().queue_allocate();
        self.obj().notify_align();
    }

    fn set_fade_width(&self, fade_width: f32) {
        if abs_diff_eq!(self.fade_width.get(), fade_width) {
            return;
        }

        self.fade_width.set(fade_width);

        self.obj().queue_allocate();
        self.obj().notify_align();
    }

    fn is_rtl(&self) -> bool {
        let direction = pango::find_base_dir(&self.get_label());

        match direction {
            pango::Direction::Rtl => true,
            pango::Direction::Ltr => false,
            _ => self.obj().direction() == gtk::TextDirection::Rtl,
        }
    }
}
