use std::cmp::Ordering;

use adw::prelude::*;
use adw::subclass::prelude::*;
use approx::abs_diff_eq;
use glib::subclass::SignalInvocationHint;
use gtk::graphene;
use num_traits as num;
use tracing_log::log::warn;

pub fn signal_accumulator_propagation(_hint: &SignalInvocationHint, return_accu: &mut glib::Value, handler_return: &glib::Value) -> bool {
    let signal_propagate = glib::Propagation::from(handler_return.get::<bool>().unwrap_or(true));

    *return_accu = handler_return.clone();
    signal_propagate.into()
}

pub trait Orthogonal {
    fn orthogonal(&self) -> Self;
}

impl Orthogonal for gtk::Orientation {
    fn orthogonal(&self) -> Self {
        match *self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
            _ => unreachable!(),
        }
    }
}

pub trait TwlWidgetExt: IsA<gtk::Widget> {
    fn iter_children(&self) -> WidgetChildIterator<Self> {
        WidgetChildIterator {
            widget: self.clone(),
            current: None,
        }
    }
}

impl TwlWidgetExt for gtk::Widget {}
impl TwlWidgetExt for gtk::Box {}

pub struct WidgetChildIterator<T: IsA<gtk::Widget>> {
    widget: T,
    current: Option<gtk::Widget>,
}

impl<T: IsA<gtk::Widget>> Iterator for WidgetChildIterator<T> {
    type Item = gtk::Widget;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = match self.current.as_ref() {
            None => self.widget.first_child(),
            Some(current) => current.next_sibling(),
        };
        self.current.clone()
    }
}

pub fn twl_widget_compute_expand(widget: &impl IsA<gtk::Widget>, hexpand: &mut bool, vexpand: &mut bool) {
    for child in widget.as_ref().iter_children() {
        *hexpand = *hexpand || child.compute_expand(gtk::Orientation::Horizontal);
        *vexpand = *vexpand || child.compute_expand(gtk::Orientation::Vertical);
    }
}

pub fn twl_widget_focus(widget: &impl IsA<gtk::Widget>, direction: gtk::DirectionType) -> bool {
    let focus_child = widget.as_ref().focus_child();

    let mut ret = false;
    for child in focus_sort(widget, direction.clone()).into_iter() {
        if focus_child.as_ref() == Some(&child) {
            ret = child.child_focus(direction.clone());
        } else if child.is_mapped() && child.is_ancestor(widget.as_ref()) {
            ret = child.child_focus(direction.clone());
        }
    }
    ret
}

pub fn twl_widget_grab_focus(widget: &impl IsA<gtk::Widget>) -> bool {
    for child in widget.as_ref().iter_children() {
        if child.grab_focus() {
            return true;
        }
    }
    false
}

fn old_focus_coords(widget: &impl IsA<gtk::Widget>) -> Option<graphene::Rect> {
    widget
        .as_ref()
        .root()
        .and_then(|r| r.focus())
        .and_then(|old_focus| old_focus.compute_bounds(widget.as_ref()))
}

/// Look for a child in @children that is intermediate between the focus widget
/// and container. This widget, if it exists, acts as the starting widget for
/// focus navigation.
fn find_old_focus(widget: &impl IsA<gtk::Widget>, children: &mut Vec<gtk::Widget>) -> Option<gtk::Widget> {
    for child in children {
        let mut test_child = child.clone();
        let mut found = true;
        while let Some(parent) = test_child.parent() {
            if &parent == widget.as_ref() {
                break;
            }

            if let Some(focus_child) = parent.focus_child() {
                if &focus_child != widget.as_ref() {
                    found = false;
                    break;
                }
            }

            test_child = parent;
        }

        if found {
            return Some(child.clone());
        }
    }

    None
}

fn focus_sort_tab(widget: &impl IsA<gtk::Widget>, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
    let text_direction = widget.as_ref().direction();
    children.sort_by(|child1, child2| {
        let child_bounds1 = child1.parent().and_then(|p1| child1.compute_bounds(&p1));
        let child_bounds2 = child2.parent().and_then(|p2| child1.compute_bounds(&p2));

        if child_bounds1.is_none() || child_bounds2.is_none() {
            return Ordering::Equal;
        }

        let child_bounds1 = child_bounds1.unwrap();
        let child_bounds2 = child_bounds2.unwrap();

        let y1 = child_bounds1.y() as f64 + (child_bounds1.height() as f64 / 2.0);
        let y2 = child_bounds2.y() as f64 + (child_bounds2.height() as f64 / 2.0);

        if abs_diff_eq!(y1, y2) {
            let x1 = child_bounds1.x() as f64 + (child_bounds1.width() as f64 / 2.0);
            let x2 = child_bounds2.x() as f64 + (child_bounds2.width() as f64 / 2.0);

            let mut inv = if text_direction == gtk::TextDirection::Rtl { -1 } else { 1 };

            if direction == gtk::DirectionType::TabBackward {
                inv = inv * -1;
            }

            let ordering = if x1 < x2 {
                -1 * inv
            } else if abs_diff_eq!(x1, x2) {
                0
            } else {
                inv
            };

            ordering.cmp(&0)
        } else {
            let mut ordering = if y1 < y2 { -1 } else { 1 };

            if direction == gtk::DirectionType::TabBackward {
                ordering = ordering * -1;
            }
            ordering.cmp(&0)
        }
    })
}

fn focus_sort_left_right(widget: &impl IsA<gtk::Widget>, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
    let old_focus = widget.as_ref().focus_child().or_else(|| find_old_focus(widget, children));

    let old_bounds = old_focus.as_ref().and_then(|w| w.compute_bounds(widget.as_ref()));

    let (compare_x, compare_y) = if let (Some(old_focus), Some(old_bounds)) = (old_focus, old_bounds) {
        // Delete widgets from list that don't match minimum criteria
        let compare_y1 = old_bounds.y();
        let compare_y2 = old_bounds.y() + old_bounds.height();

        let compare_x = if direction == gtk::DirectionType::Left {
            old_bounds.x()
        } else {
            old_bounds.x() + old_bounds.width()
        };

        *children = children
            .iter()
            .filter(|child| {
                if *child != &old_focus {
                    if let Some(child_bounds) = child.compute_bounds(widget.as_ref()) {
                        let child_y1 = child_bounds.y();
                        let child_y2 = child_bounds.y() + child_bounds.height();

                        if abs_diff_eq!(child_y2, compare_y1) || child_y2 < compare_y1 ||
                       abs_diff_eq!(child_y1, compare_y2) || child_y1 > compare_y2 /* No vertical overlap */ ||
                       (direction == gtk::DirectionType::Right && (child_bounds.x() + child_bounds.width()) < compare_x) || /* Not to left */
                       (direction == gtk::DirectionType::Left && (child_bounds.x() > compare_x))
                        /* Not to right */
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        (old_bounds.x() + (old_bounds.width() / 2.0), (compare_y1 + compare_y2) / 2.0)
    } else {
        // No old focus widget, need to figure out starting x,y some other way

        let bounds = widget
            .as_ref()
            .compute_bounds(widget.as_ref().parent().as_ref().unwrap_or(widget.as_ref().upcast_ref()))
            .unwrap_or(graphene::Rect::new(0.0, 0.0, 0.0, 0.0));
        let compare_y = if let Some(old_focus_bounds) = old_focus_coords(widget) {
            old_focus_bounds.y() + (old_focus_bounds.height() / 2.0)
        } else if widget.as_ref().native().is_none() {
            bounds.y() + (bounds.height() / 2.0)
        } else {
            bounds.height() / 2.0
        };

        let compare_x = if widget.as_ref().native().is_none() {
            if direction == gtk::DirectionType::Right {
                bounds.x()
            } else {
                bounds.x() + bounds.width()
            }
        } else {
            if direction == gtk::DirectionType::Left {
                0.0
            } else {
                bounds.width()
            }
        };

        (compare_x, compare_y)
    };

    let reverse = direction == gtk::DirectionType::Left;

    children.sort_by(|child1, child2| axis_compare(widget, child1, child2, compare_x, compare_y, reverse, gtk::Orientation::Horizontal))
}

fn focus_sort_up_down(widget: &impl IsA<gtk::Widget>, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
    let old_focus = widget.as_ref().focus_child().or_else(|| find_old_focus(widget, children));

    let old_bounds = old_focus.as_ref().and_then(|w| w.compute_bounds(widget.as_ref()));
    let (compare_x, compare_y) = if let (Some(old_focus), Some(old_bounds)) = (old_focus, old_bounds) {
        // Delete widgets from list that don't match minimum criteria
        let compare_x1 = old_bounds.x();
        let compare_x2 = old_bounds.x() + old_bounds.width();

        let compare_y = if direction == gtk::DirectionType::Up {
            old_bounds.y()
        } else {
            old_bounds.y() + old_bounds.height()
        };

        *children = children
            .iter()
            .filter(|child| {
                if *child != &old_focus {
                    if let Some(child_bounds) = child.compute_bounds(widget.as_ref()) {
                        let child_x1 = child_bounds.x();
                        let child_x2 = child_bounds.x() + child_bounds.width();

                        if abs_diff_eq!(child_x2, compare_x1) || child_x2 < compare_x1 ||
                       abs_diff_eq!(child_x1, compare_x2) || child_x1 > compare_x2 /* No horizontal overlap */ ||
                       (direction == gtk::DirectionType::Down && (child_bounds.y() + child_bounds.height()) < compare_y) || /* Not below */
                       (direction == gtk::DirectionType::Up && (child_bounds.y() > compare_y))
                        /* Not above */
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        ((compare_x1 + compare_x2) / 2.0, old_bounds.y() + (old_bounds.height() / 2.0))
    } else {
        // No old focus widget, need to figure out starting x,y some other way

        let bounds = widget
            .as_ref()
            .compute_bounds(widget.as_ref().parent().as_ref().unwrap_or(widget.as_ref().upcast_ref()))
            .unwrap_or(graphene::Rect::new(0.0, 0.0, 0.0, 0.0));
        let compare_x = if let Some(old_focus_bounds) = old_focus_coords(widget) {
            old_focus_bounds.x() + (old_focus_bounds.width() / 2.0)
        } else if widget.as_ref().native().is_none() {
            bounds.x() + (bounds.width() / 2.0)
        } else {
            bounds.width() / 2.0
        };

        let compare_y = if widget.as_ref().native().is_none() {
            if direction == gtk::DirectionType::Down {
                bounds.y()
            } else {
                bounds.y() + bounds.height()
            }
        } else {
            if direction == gtk::DirectionType::Down {
                0.0
            } else {
                bounds.height()
            }
        };

        (compare_x, compare_y)
    };

    let reverse = direction == gtk::DirectionType::Up;

    children.sort_by(|child1, child2| axis_compare(widget, child1, child2, compare_x, compare_y, reverse, gtk::Orientation::Vertical))
}

fn focus_sort(widget: &impl IsA<gtk::Widget>, direction: gtk::DirectionType) -> Vec<gtk::Widget> {
    // Initialize the list with all visible child widgets
    let mut children: Vec<gtk::Widget> = widget.as_ref().iter_children().filter(|c| c.is_mapped() && c.is_sensitive()).collect();

    //  Now sort that list depending on @direction
    match direction {
        gtk::DirectionType::TabForward | gtk::DirectionType::TabBackward => focus_sort_tab(widget, &mut children, direction),
        gtk::DirectionType::Up | gtk::DirectionType::Down => focus_sort_up_down(widget, &mut children, direction),
        gtk::DirectionType::Left | gtk::DirectionType::Right => focus_sort_left_right(widget, &mut children, direction),
        _ => unreachable!("unknown direction type"),
    }

    children
}

fn axis_compare(
    widget: &impl IsA<gtk::Widget>,
    child1: &impl IsA<gtk::Widget>,
    child2: &impl IsA<gtk::Widget>,
    x: f32,
    y: f32,
    reverse: bool,
    orientation: gtk::Orientation,
) -> Ordering {
    let bounds1 = child1.as_ref().compute_bounds(widget.as_ref());
    let bounds2 = child2.as_ref().compute_bounds(widget.as_ref());

    if bounds1.is_none() || bounds2.is_none() {
        return Ordering::Equal;
    }

    let (mut start1, end1) = axis_info(bounds1.as_ref().unwrap(), orientation);
    let (mut start2, end2) = axis_info(bounds2.as_ref().unwrap(), orientation);

    start1 = start1 + (end1 / 2.0);
    start2 = start2 + (end2 / 2.0);

    let (x1, x2) = if start1 == start2 {
        //  Now use origin/bounds to compare the 2 widgets on the other axis
        let (start1, end1) = axis_info(bounds1.as_ref().unwrap(), orientation.orthogonal());
        let (start2, end2) = axis_info(bounds2.as_ref().unwrap(), orientation.orthogonal());

        let x1 = num::abs(start1 + (end1 / 2.0) - x);
        let x2 = num::abs(start2 + (end2 / 2.0) - x);

        (x1, x2)
    } else {
        (start1, start2)
    };

    let inv = if reverse { -1 } else { 1 };
    let ordering = if x1 < x2 {
        -1 * inv
    } else if abs_diff_eq!(x1, x2) {
        0
    } else {
        inv
    };
    ordering.cmp(&0)
}

fn axis_info(bounds: &graphene::Rect, orientation: gtk::Orientation) -> (f32, f32) {
    match orientation {
        gtk::Orientation::Horizontal => (bounds.x(), bounds.width()),
        gtk::Orientation::Vertical => (bounds.y(), bounds.height()),
        _ => unreachable!(),
    }
}

// gboolean
// adw_widget_grab_focus_self (GtkWidget *widget)
// {
//   if (!gtk_widget_get_focusable (widget))
//     return FALSE;

//   gtk_root_set_focus (gtk_widget_get_root (widget), widget);

//   return TRUE;
// }

// gboolean
// adw_widget_grab_focus_child (GtkWidget *widget)
// {
//   GtkWidget *child;

//   for (child = gtk_widget_get_first_child (widget);
//        child != NULL;
//        child = gtk_widget_get_next_sibling (child))
//     if (gtk_widget_grab_focus (child))
//       return TRUE;

//   return FALSE;
// }

// void
// adw_widget_compute_expand (GtkWidget *widget,
//                            gboolean  *hexpand_p,
//                            gboolean  *vexpand_p)
// {
//   GtkWidget *child;
//   gboolean hexpand = FALSE;
//   gboolean vexpand = FALSE;

//   for (child = gtk_widget_get_first_child (widget);
//        child != NULL;
//        child = gtk_widget_get_next_sibling (child)) {
//     hexpand = hexpand || gtk_widget_compute_expand (child, GTK_ORIENTATION_HORIZONTAL);
//     vexpand = vexpand || gtk_widget_compute_expand (child, GTK_ORIENTATION_VERTICAL);
//   }

//   *hexpand_p = hexpand;
//   *vexpand_p = vexpand;
// }

// void
// adw_widget_compute_expand_horizontal_only (GtkWidget *widget,
//                                            gboolean  *hexpand_p,
//                                            gboolean  *vexpand_p)
// {
//   GtkWidget *child;
//   gboolean hexpand = FALSE;

//   for (child = gtk_widget_get_first_child (widget);
//        child != NULL;
//        child = gtk_widget_get_next_sibling (child))
//     hexpand = hexpand || gtk_widget_compute_expand (child, GTK_ORIENTATION_HORIZONTAL);

//   *hexpand_p = hexpand;
//   *vexpand_p = FALSE;
// }

// GtkSizeRequestMode
// adw_widget_get_request_mode (GtkWidget *widget)
// {
//   GtkWidget *child;
//   int wfh = 0, hfw = 0;

//   for (child = gtk_widget_get_first_child (widget);
//        child;
//        child = gtk_widget_get_next_sibling (child)) {
//     GtkSizeRequestMode mode = gtk_widget_get_request_mode (child);

//     switch (mode) {
//     case GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH:
//       hfw++;
//       break;
//     case GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT:
//       wfh++;
//       break;
//     case GTK_SIZE_REQUEST_CONSTANT_SIZE:
//     default:
//       break;
//     }
//   }

//   if (hfw == 0 && wfh == 0)
//     return GTK_SIZE_REQUEST_CONSTANT_SIZE;
//   else
//     return wfh > hfw ?
//         GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT :
//         GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH;
// }

// /* FIXME: Replace this with public color API and make public */
// gboolean
// adw_widget_lookup_color (GtkWidget  *widget,
//                          const char *name,
//                          GdkRGBA    *rgba)
// {
// G_GNUC_BEGIN_IGNORE_DEPRECATIONS
//   GtkStyleContext *context = gtk_widget_get_style_context (widget);

//   return gtk_style_context_lookup_color (context, name, rgba);
// G_GNUC_END_IGNORE_DEPRECATIONS
// }

// GtkWidget *
// adw_widget_get_ancestor (GtkWidget *widget,
//                          GType      widget_type,
//                          gboolean   same_native,
//                          gboolean   same_sheet)
// {
//   while (widget && !g_type_is_a (G_OBJECT_TYPE (widget), widget_type)) {
//     if (same_native && GTK_IS_NATIVE (widget))
//       return NULL;

//     if (same_sheet && (ADW_IS_FLOATING_SHEET (widget) || ADW_IS_BOTTOM_SHEET (widget)))
//       return NULL;

//     widget = gtk_widget_get_parent (widget);
//   }

//   return widget;
// }

// gboolean
// adw_decoration_layout_prefers_start (const char *layout)
// {
//   int counts[2];
//   char **sides;
//   int i;

//   sides = g_strsplit (layout, ":", 2);

//   for (i = 0; i < 2; i++) {
//     char **elements;
//     int j;

//     counts[i] = 0;

//     if (sides[i] == NULL)
//       continue;

//     elements = g_strsplit (sides[i], ",", -1);

//     for (j = 0; elements[j]; j++) {
//       if (!g_strcmp0 (elements[j], "close"))
//         counts[i]++;
//     }

//     g_strfreev (elements);
//   }

//   g_strfreev (sides);

//   return counts[0] > counts[1];
// }
