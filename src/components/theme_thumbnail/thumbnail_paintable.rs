/// This file is derived work from
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
use elementtree::Element;
use glib::{prelude::*, subclass::prelude::*};
use gtk::graphene;
use gtk::prelude::*;
use once_cell::sync::Lazy;
use rand::Rng;
use ref_thread_local::{ref_thread_local, RefThreadLocal};
use tracing::*;

use gdk::subclass::prelude::*;

use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};

use crate::theme_provider::Theme;

ref_thread_local! {
    static managed INSTANCE: Lazy<ThemeThumbnailProvider> = Lazy::new(ThemeThumbnailProvider::new);
}

impl Default for ThemeThumbnailProvider {
    fn default() -> Self {
        INSTANCE.borrow().clone()
    }
}

#[derive(Clone)]
struct ThemeThumbnailProvider {
    element: Option<Element>,
}

impl ThemeThumbnailProvider {
    fn new() -> Self {
        let file = gio::File::for_uri("resource:///io/github/vhdirk/Terms/svg/theme-thumbnail.svg");
        let element = file
            .load_contents(None::<&gio::Cancellable>)
            .map_err(|err| warn!("Could not load theme-thumbnail.svg: {:?}", err))
            .and_then(|(bytes, _opt)| Element::from_reader(bytes.as_slice()).map_err(|err| warn!("Could not parse theme-thumbnail.svg: {:?}", err)))
            .ok();

        Self { element }
    }

    fn process_node(elem: &mut Element, theme: &Theme) {
        let mut color = None;
        if elem.get_attr("label") == Some("palette") {
            if let Some(palette) = &theme.palette {
                let mut rng = rand::thread_rng();
                let random_number = rng.gen_range(7..palette.len());
                color = palette.get(random_number).cloned();
            }
        }

        if elem.get_attr("label") == Some("fg") {
            color = theme.foreground;
        }

        if let Some(color) = color {
            let style = format!("stroke:{};stroke-width:3;stroke-linecap:round;", color);
            elem.set_attr("style", style);
        }

        for child in elem.children_mut() {
            Self::process_node(child, theme);
        }
    }

    pub fn apply_theme(&self, theme: &Theme) -> Option<String> {
        info!("Rerendering svg in theme {}", theme.name);

        self.element.as_ref()?;

        let mut element = self.element.as_ref().unwrap().clone();
        Self::process_node(&mut element, theme);
        element.to_string().map_err(|err| warn!("Could render svg {:?}", err)).ok()
    }
}

mod imp {
    use gdk::cairo::Rectangle;
    use once_cell::sync::OnceCell;
    use rsvg::{CairoRenderer, SvgHandle};

    use super::*;

    #[derive(Default)]
    pub struct ThemePreviewPaintable {
        handle: OnceCell<SvgHandle>,
        theme: OnceCell<Theme>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThemePreviewPaintable {
        const NAME: &'static str = "TermsThemePreviewPaintable";
        type Type = super::ThemePreviewPaintable;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for ThemePreviewPaintable {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl PaintableImpl for ThemePreviewPaintable {
        fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
            let snapshot: &gtk::Snapshot = match snapshot.downcast_ref() {
                Some(snapshot) => snapshot,
                None => {
                    warn!("Could not downcast snapshot");
                    return;
                },
            };

            let ctx = snapshot.append_cairo(&graphene::Rect::new(0.0, 0.0, width as f32, height as f32));

            if let Some(handle) = self.get_handle() {
                let renderer = CairoRenderer::new(handle);
                match renderer.render_document(&ctx, &Rectangle::new(0.0, 0.0, width, height)) {
                    Ok(_) => (),
                    Err(err) => warn!("Could not render svg: {:?}", err),
                }
            }
        }
    }

    impl ThemePreviewPaintable {
        pub fn set_theme(&self, theme: &Theme) {
            let _ = self.theme.set(theme.clone());
        }

        pub fn get_handle(&self) -> Option<&SvgHandle> {
            if self.handle.get().is_none() {
                if let Some(themed) = self.theme.clone().get().as_ref().and_then(|t| ThemeThumbnailProvider::default().apply_theme(t)) {
                    let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from_owned(themed));
                    match rsvg::Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>) {
                        Ok(handle) => {
                            let _ = self.handle.set(handle);
                        },
                        Err(err) => warn!("Could not load svg {:?}", err),
                    }
                }
            }
            self.handle.get()
        }
    }
}

glib::wrapper! {
    pub struct ThemePreviewPaintable(ObjectSubclass<imp::ThemePreviewPaintable>) @implements gdk::Paintable;
}

impl ThemePreviewPaintable {
    pub fn new(theme: &Theme) -> Self {
        let this: Self = glib::Object::builder().build();
        this.imp().set_theme(theme);
        this
    }
}
