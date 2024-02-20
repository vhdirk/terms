use std::cmp::Ordering;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib;
use gtk::graphene;

use approx::abs_diff_eq;
use num_traits as num;

use super::utils::{Orthogonal, TwlWidgetExt};

#[derive(Debug, Default)]
pub struct Bin {}

#[glib::object_subclass]
impl ObjectSubclass for Bin {
    const NAME: &'static str = "TwlBin";
    type Type = super::Bin;
    type ParentType = adw::Bin;
}

impl ObjectImpl for Bin {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for Bin {
    fn focus(&self, direction: gtk::DirectionType) -> bool {
        let focus_child = self.obj().focus_child();

        let mut ret = false;
        for child in self.focus_sort(direction.clone()).into_iter() {
            if focus_child.as_ref() == Some(&child) {
                ret = child.child_focus(direction.clone());
            } else if child.is_mapped() && child.is_ancestor(&*self.obj()) {
                ret = child.child_focus(direction.clone());
            }
        }
        ret
    }

    fn grab_focus(&self) -> bool {
        for child in self.obj().iter_children() {
            if child.grab_focus() {
                return true;
            }
        }
        false
    }
}

impl BinImpl for Bin {}

impl Bin {
    fn old_focus_coords(&self) -> Option<graphene::Rect> {
        self.obj()
            .root()
            .and_then(|r| r.focus())
            .and_then(|old_focus| old_focus.compute_bounds(self.obj().as_ref()))
    }

    /// Look for a child in @children that is intermediate between the focus widget
    /// and container. This widget, if it exists, acts as the starting widget for
    /// focus navigation.
    fn find_old_focus(&self, children: &mut Vec<gtk::Widget>) -> Option<gtk::Widget> {
        for child in children {
            let mut test_child = child.clone();
            let mut found = true;
            while let Some(parent) = test_child.parent() {
                if parent == *self.obj() {
                    break;
                }

                if let Some(focus_child) = parent.focus_child() {
                    if focus_child != *self.obj() {
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

    pub fn focus_sort_tab(&self, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
        let text_direction = self.obj().direction();
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

    pub fn focus_sort_left_right(&self, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
        let old_focus = self.obj().focus_child().or_else(|| self.find_old_focus(children));

        let old_bounds = old_focus.as_ref().and_then(|w| w.compute_bounds(self.obj().as_ref()));

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
                        if let Some(child_bounds) = child.compute_bounds(self.obj().as_ref()) {
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

            let bounds = self
                .obj()
                .compute_bounds(self.obj().parent().as_ref().unwrap_or(self.obj().upcast_ref()))
                .unwrap_or(graphene::Rect::new(0.0, 0.0, 0.0, 0.0));
            let compare_y = if let Some(old_focus_bounds) = self.old_focus_coords() {
                old_focus_bounds.y() + (old_focus_bounds.height() / 2.0)
            } else if self.obj().native().is_none() {
                bounds.y() + (bounds.height() / 2.0)
            } else {
                bounds.height() / 2.0
            };

            let compare_x = if self.obj().native().is_none() {
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

        children.sort_by(|child1, child2| self.axis_compare(child1, child2, compare_x, compare_y, reverse, gtk::Orientation::Horizontal))
    }

    pub fn focus_sort_up_down(&self, children: &mut Vec<gtk::Widget>, direction: gtk::DirectionType) {
        let old_focus = self.obj().focus_child().or_else(|| self.find_old_focus(children));

        let old_bounds = old_focus.as_ref().and_then(|w| w.compute_bounds(self.obj().as_ref()));
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
                        if let Some(child_bounds) = child.compute_bounds(self.obj().as_ref()) {
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

            let bounds = self
                .obj()
                .compute_bounds(self.obj().parent().as_ref().unwrap_or(self.obj().upcast_ref()))
                .unwrap_or(graphene::Rect::new(0.0, 0.0, 0.0, 0.0));
            let compare_x = if let Some(old_focus_bounds) = self.old_focus_coords() {
                old_focus_bounds.x() + (old_focus_bounds.width() / 2.0)
            } else if self.obj().native().is_none() {
                bounds.x() + (bounds.width() / 2.0)
            } else {
                bounds.width() / 2.0
            };

            let compare_y = if self.obj().native().is_none() {
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

        children.sort_by(|child1, child2| self.axis_compare(child1, child2, compare_x, compare_y, reverse, gtk::Orientation::Vertical))
    }

    fn focus_sort(&self, direction: gtk::DirectionType) -> Vec<gtk::Widget> {
        // Initialize the list with all visible child widgets
        let mut children: Vec<gtk::Widget> = self.obj().iter_children().filter(|c| c.is_mapped() && c.is_sensitive()).collect();

        //  Now sort that list depending on @direction
        match direction {
            gtk::DirectionType::TabForward | gtk::DirectionType::TabBackward => self.focus_sort_tab(&mut children, direction),
            gtk::DirectionType::Up | gtk::DirectionType::Down => self.focus_sort_up_down(&mut children, direction),
            gtk::DirectionType::Left | gtk::DirectionType::Right => self.focus_sort_left_right(&mut children, direction),
            _ => unreachable!("unknown direction type"),
        }

        children
    }

    fn axis_compare(
        &self,
        child1: &impl IsA<gtk::Widget>,
        child2: &impl IsA<gtk::Widget>,
        x: f32,
        y: f32,
        reverse: bool,
        orientation: gtk::Orientation,
    ) -> Ordering {
        let bounds1 = child1.as_ref().compute_bounds(self.obj().as_ref());
        let bounds2 = child2.as_ref().compute_bounds(self.obj().as_ref());

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

    // gboolean
    // adw_widget_grab_focus_self (GtkWidget *widget)
    // {
    //   if (!gtk_widget_get_focusable (widget))
    //     return FALSE;

    //   gtk_root_set_focus (gtk_widget_get_root (widget), widget);

    //   return TRUE;
    // }
}

fn axis_info(bounds: &graphene::Rect, orientation: gtk::Orientation) -> (f32, f32) {
    match orientation {
        gtk::Orientation::Horizontal => (bounds.x(), bounds.width()),
        gtk::Orientation::Vertical => (bounds.y(), bounds.height()),
        _ => unreachable!(),
    }
}
