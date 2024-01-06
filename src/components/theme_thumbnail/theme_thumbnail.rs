use glib::clone;
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
use glib::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::RefCell;

use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::thumbnail_paintable::ThemePreviewPaintable;
use crate::services::theme_provider::Theme;

#[derive(Debug, Default)]
pub struct ThemeThumbnailCtx {
    pub selected: bool,
    pub theme: Option<Theme>,
    pub picture: Option<gtk::Picture>,
    pub check_icon: Option<gtk::Image>,
}

#[derive(Debug, Default)]
pub struct ThemeThumbnail {
    pub ctx: RefCell<ThemeThumbnailCtx>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeThumbnail {
    const NAME: &'static str = "TermsThemeThumbnail";
    type Type = super::ThemeThumbnail;
    type ParentType = gtk::FlowBoxChild;
}

impl ObjectImpl for ThemeThumbnail {
    fn constructed(&self) {
        self.parent_constructed();

        let img = gtk::Picture::builder()
            .width_request(110)
            .height_request(70)
            .css_classes(["card"])
            .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
            .build();
        img.set_parent(&self.obj().clone());
        self.ctx.borrow_mut().picture = Some(img);

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
        self.ctx.borrow_mut().check_icon = Some(check_icon);

        // Emit activate signal when thumbnail is clicked.
        let mouse_control = gtk::GestureClick::builder().build();
        mouse_control.connect_pressed(clone!(@weak self as this => move |_, _, _, _| {
            if !this.ctx.borrow().selected {
                this.set_selected(true);
                this.obj().notify("selected");
            }
        }));

        self.obj().add_controller(mouse_control);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| vec![glib::ParamSpecBoolean::builder("selected").readwrite().build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "selected" => {
                if let Ok(selected) = value.get::<bool>() {
                    self.set_selected(selected);
                }
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "selected" => self.ctx.borrow().selected.to_value(),
            _ => unimplemented!(),
        }
    }

    fn dispose(&self) {
        if let Some(picture) = self.ctx.borrow_mut().picture.take() {
            picture.unparent();
        }
        if let Some(check_icon) = self.ctx.borrow_mut().check_icon.take() {
            check_icon.unparent();
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

        self.ctx.borrow_mut().theme = Some(theme.clone());

        let paintable: ThemePreviewPaintable = ThemePreviewPaintable::new(theme);
        if let Some(img) = self.ctx.borrow().picture.as_ref() {
            img.set_paintable(Some(&paintable));

            if let Some(bg_color) = theme.background_color {
                let css_provider = gtk::CssProvider::new();
                css_provider.load_from_data(&format!("picture {{ background-color: {}; }}", bg_color));
                img.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
        }
    }

    fn set_selected(&self, selected: bool) {
        self.ctx.borrow_mut().selected = selected;
        if let Some(picture) = self.ctx.borrow().picture.as_ref() {
            if selected {
                picture.add_css_class("selected");
            } else {
                picture.remove_css_class("selected");
            }
        }

        if let Some(check_icon) = self.ctx.borrow().check_icon.as_ref() {
            check_icon.set_visible(selected)
        }
    }
}
