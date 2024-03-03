use once_cell::sync::OnceCell;
/// ColorSchemeThumbnail.vala
///
/// Copyright 2021-2022 Paulo Queiroz
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.
///
use std::cell::Cell;

use convert_case::{Case, Casing};
use glib::{clone, Properties};
use gtk::{prelude::*, subclass::prelude::*};

use super::thumbnail_paintable::ThemePreviewPaintable;
use crate::theme_provider::Theme;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type=super::ThemeThumbnail)]
pub struct ThemeThumbnail {
    #[property(get, set=Self::set_selected, construct)]
    pub selected: Cell<bool>,

    pub theme: OnceCell<Theme>,
    pub picture: OnceCell<gtk::Picture>,
    pub check_icon: OnceCell<gtk::Image>,
    pub css_provider: OnceCell<gtk::CssProvider>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeThumbnail {
    const NAME: &'static str = "TermsThemeThumbnail";
    type Type = super::ThemeThumbnail;
    type ParentType = gtk::FlowBoxChild;
}

#[glib::derived_properties]
impl ObjectImpl for ThemeThumbnail {
    fn constructed(&self) {
        self.parent_constructed();

        let picture = gtk::Picture::builder()
            .width_request(110)
            .height_request(70)
            .css_classes(["card"])
            .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
            .build();
        picture.set_parent(&self.obj().clone());
        self.picture.set(picture).unwrap();

        // Icon will show when this.selected is true
        let check_icon = gtk::Image::builder()
            .icon_name("object-select-symbolic")
            .pixel_size(14)
            .vexpand(true)
            .valign(gtk::Align::End)
            .halign(gtk::Align::End)
            .visible(false)
            .build();

        check_icon.set_parent(&self.obj().clone());
        self.check_icon.set(check_icon).unwrap();

        // Emit activate signal when thumbnail is clicked.
        let mouse_control = gtk::GestureClick::builder().build();
        mouse_control.connect_pressed(clone!(@weak self as this => move |_, _, _, _| {
            if !this.selected.get() {
                this.obj().set_selected(true);
            }
        }));

        self.obj().add_controller(mouse_control);
    }

    fn dispose(&self) {
        if let Some(picture) = self.picture.get() {
            picture.unparent();
        }
        if let Some(check_icon) = self.check_icon.get() {
            check_icon.unparent();
        }
        if let Some(css_provider) = self.css_provider.get() {
            gtk::style_context_remove_provider_for_display(&self.obj().display(), css_provider);
        }
    }
}

impl WidgetImpl for ThemeThumbnail {}

impl FlowBoxChildImpl for ThemeThumbnail {}

impl ThemeThumbnail {
    pub fn set_theme(&self, theme: &Theme) {
        self.obj().add_css_class("thumbnail");
        self.obj().set_has_tooltip(true);
        self.obj().set_tooltip_text(Some(&theme.name));

        self.theme.set(theme.clone()).unwrap();

        let paintable: ThemePreviewPaintable = ThemePreviewPaintable::new(theme);
        if let Some(pic) = self.picture.get() {
            pic.set_paintable(Some(&paintable));

            let css_class = theme.name.to_case(Case::Snake);
            pic.add_css_class(&css_class);

            if let Some(bg_color) = theme.background {
                let css_provider = gtk::CssProvider::new();
                css_provider.load_from_string(&format!("picture.{} {{ background-color: {}; }}", css_class, bg_color));

                gtk::style_context_add_provider_for_display(&self.obj().display(), &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
                self.css_provider.set(css_provider).unwrap();
            }
        }
    }

    fn set_selected(&self, selected: bool) {
        self.selected.set(selected);
        if let Some(picture) = self.picture.get() {
            if selected {
                picture.add_css_class("selected");
            } else {
                picture.remove_css_class("selected");
            }
        }

        if let Some(check_icon) = self.check_icon.get() {
            check_icon.set_visible(selected)
        }
    }
}
