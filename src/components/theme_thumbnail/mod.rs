use glib::subclass::prelude::*;

use crate::theme_provider::Theme;

mod imp;
mod thumbnail_paintable;

glib::wrapper! {
    /// Thumbnail of color scheme
    /// Based on GtkSourceStyleSchemePreview:
    /// https://gitlab.gnome.org/GNOME/gtksourceview/-/blob/master/gtksourceview/gtksourcestyleschemepreview.c
    pub struct ThemeThumbnail(ObjectSubclass<imp::ThemeThumbnail>) @extends gtk::Widget, gtk::FlowBoxChild,  @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ThemeThumbnail {
    pub fn new(theme: &Theme) -> Self {
        let this: Self = glib::Object::builder().build();
        this.imp().set_theme(theme);
        this
    }

    pub fn theme(&self) -> Option<Theme> {
        self.imp().theme.get().cloned()
    }
}
