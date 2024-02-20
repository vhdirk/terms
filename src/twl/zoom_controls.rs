use std::path::PathBuf;

use super::zoom_controls_imp as imp;
use glib::closure_local;
use gtk::prelude::*;

glib::wrapper! {
        pub struct ZoomControls(ObjectSubclass<imp::ZoomControls>)
                @extends gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ZoomControls {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

//  /* Add zoom controls */
//   zoom_box = g_object_new (GTK_TYPE_BOX,
//                            "spacing", 12,
//                            "margin-start", 18,
//                            "margin-end", 18,
//                            NULL);
//   zoom_in = g_object_new (GTK_TYPE_BUTTON,
//                           "action-name", "win.zoom-in",
//                           "action-target", g_variant_new_boolean (FALSE),
//                           "tooltip-text", _("Zoom In"),
//                           "child", g_object_new (GTK_TYPE_IMAGE,
//                                                  "icon-name", "zoom-in-symbolic",
//                                                  "pixel-size", 16,
//                                                  NULL),
//                           NULL);
//   gtk_widget_add_css_class (zoom_in, "circular");
//   gtk_widget_add_css_class (zoom_in, "flat");
//   gtk_widget_set_tooltip_text (zoom_in, _("Zoom In"));
//   gtk_accessible_update_property (GTK_ACCESSIBLE (zoom_in),
//                                   GTK_ACCESSIBLE_PROPERTY_LABEL,
//                                   _("Zoom in"), -1);
//   zoom_out = g_object_new (GTK_TYPE_BUTTON,
//                            "action-name", "win.zoom-out",
//                            "action-target", g_variant_new_boolean (FALSE),
//                            "tooltip-text", _("Zoom Out"),
//                            "child", g_object_new (GTK_TYPE_IMAGE,
//                                                   "icon-name", "zoom-out-symbolic",
//                                                   "pixel-size", 16,
//                                                   NULL),
//                            NULL);
//   gtk_widget_add_css_class (zoom_out, "circular");
//   gtk_widget_add_css_class (zoom_out, "flat");
//   gtk_widget_set_tooltip_text (zoom_out, _("Zoom Out"));
//   gtk_accessible_update_property (GTK_ACCESSIBLE (zoom_out),
//                                   GTK_ACCESSIBLE_PROPERTY_LABEL,
//                                   _("Zoom out"), -1);
//   self->zoom_label = g_object_new (GTK_TYPE_BUTTON,
//                                    "css-classes", (const char * const[]) {"flat", "pill", NULL},
//                                    "action-name", "win.zoom-one",
//                                    "action-target", g_variant_new_boolean (FALSE),
//                                    "hexpand", TRUE,
//                                    "tooltip-text", _("Reset Zoom"),
//                                    "label", "100%",
//                                    NULL);
//   g_binding_group_bind (self->active_tab_bindings, "zoom-label", self->zoom_label, "label", 0);
//   gtk_box_append (GTK_BOX (zoom_box), zoom_out);
//   gtk_box_append (GTK_BOX (zoom_box), self->zoom_label);
//   gtk_box_append (GTK_BOX (zoom_box), zoom_in);
//   gtk_popover_menu_add_child (GTK_POPOVER_MENU (popover), zoom_box, "zoom");
