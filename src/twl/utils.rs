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
