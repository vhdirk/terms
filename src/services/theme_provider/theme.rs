use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

use serde::{Deserialize, Serialize};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub comment: Option<String>,

    #[serde(rename = "use-theme-colors")]
    pub use_theme_colors: Option<bool>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "foreground-color")]
    pub foreground_color: Option<gdk::RGBA>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "background-color")]
    pub background_color: Option<gdk::RGBA>,

    #[serde_as(as = "Option<[DisplayFromStr; 16]>")]
    pub palette: Option<[gdk::RGBA; 16]>,
}

impl Theme {
    fn color_brightness(color: &gdk::RGBA) -> f64 {
        ((color.red() as f64 * 299.0) + (color.green() as f64 * 587.0) + (color.blue() as f64 * 114.0)) / 1000.0
    }

    pub fn is_dark(&self) -> bool {
        if let Some(color) = self.foreground_color {
            Self::color_brightness(&color) > 0.5
        } else if let Some(color) = self.background_color {
            Self::color_brightness(&color) <= 0.5
        } else {
            true
        }
    }
}
