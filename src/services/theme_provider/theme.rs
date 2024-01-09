use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use std::{ffi::OsStr, fs, path::Path};
use tracing::*;

use serde::{Deserialize, Serialize};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThemeTemp {
    pub name: String,
    pub comment: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub foreground: Option<gdk::RGBA>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub background: Option<gdk::RGBA>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub cursor: Option<gdk::RGBA>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_01: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_02: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_03: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_04: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_05: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_06: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_07: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_08: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_09: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_10: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_11: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_12: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_13: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_14: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_15: Option<gdk::RGBA>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub color_16: Option<gdk::RGBA>,
}

impl ThemeTemp {
    pub fn palette(&self) -> Option<Vec<gdk::RGBA>> {
        let colors = vec![
            self.color_01,
            self.color_02,
            self.color_03,
            self.color_04,
            self.color_05,
            self.color_06,
            self.color_07,
            self.color_08,
            self.color_09,
            self.color_10,
            self.color_11,
            self.color_12,
            self.color_13,
            self.color_14,
            self.color_15,
            self.color_16,
        ];

        if colors.iter().any(|c| c.is_none()) {
            return None;
        }

        Some(colors.iter().map(|c| c.unwrap()).collect())
    }
}

impl Into<Theme> for ThemeTemp {
    fn into(self) -> Theme {
        Theme {
            name: self.name.clone(),
            comment: self.comment.clone(),
            foreground: self.foreground.clone(),
            background: self.background.clone(),
            cursor: self.cursor.clone(),
            palette: self.palette().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub comment: Option<String>,

    pub foreground: Option<gdk::RGBA>,
    pub background: Option<gdk::RGBA>,
    pub cursor: Option<gdk::RGBA>,

    pub palette: Option<Vec<gdk::RGBA>>,
}

impl Theme {
    pub fn from_file(file_path: &Path) -> Option<Self> {
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                error!("Error while reading color theme file {}: {}", file_path.to_string_lossy(), err);
                return None;
            },
        };

        let ext = file_path.extension().map(|ext| ext.to_string_lossy().to_string());
        if ext.is_none() {
            return None;
        }
        let ext = ext.unwrap();
        match ext.as_str() {
            "yml" | "yaml" => serde_yaml::from_str(&content)
                .map_err(|err| error!("Error while reading color theme file {}: {}", file_path.to_string_lossy(), err))
                .ok(),
            "json" => serde_json::from_str(&content)
                .map_err(|err| error!("Error while reading color theme file {}: {}", file_path.to_string_lossy(), err))
                .ok(),
            _ => unimplemented!(),
        }
        .map(|t: ThemeTemp| t.into())
    }

    fn color_brightness(color: &gdk::RGBA) -> f64 {
        ((color.red() as f64 * 299.0) + (color.green() as f64 * 587.0) + (color.blue() as f64 * 114.0)) / 1000.0
    }

    pub fn is_dark(&self) -> bool {
        if let Some(color) = self.background {
            Self::color_brightness(&color) <= 0.5
        } else if let Some(color) = self.foreground {
            Self::color_brightness(&color) > 0.5
        } else {
            true
        }
    }
}
