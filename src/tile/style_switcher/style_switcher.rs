use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, CompositeTemplate};
/// This file is work derived from Black Box
///
/// Copyright 2023 Paulo Queiroz <pvaqueiroz@gmail.com>
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
/// SPDX-License-Identifier: GPL-3.0-or-later
///
use std::cell::RefCell;
use tracing::*;

#[derive(Debug, Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Tile/gtk/style_switcher.ui")]
#[properties(wrapper_type=super::StyleSwitcher)]
pub struct StyleSwitcher {
    #[template_child]
    pub system_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub light_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_selector: TemplateChild<gtk::CheckButton>,

    #[property(get, set=Self::set_style_preference ,construct, default="system")]
    pub preference: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for StyleSwitcher {
    const NAME: &'static str = "TileStyleSwitcher";
    type Type = super::StyleSwitcher;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
#[glib::derived_properties]
impl ObjectImpl for StyleSwitcher {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for StyleSwitcher {}

#[gtk::template_callbacks]
impl StyleSwitcher {
    fn set_style_preference(&self, preference: &str) {
        let _system_guard = self.system_selector.freeze_notify();
        let _light_guard = self.light_selector.freeze_notify();
        let _dark_guard = self.dark_selector.freeze_notify();

        match preference {
            "system" => {
                self.system_selector.set_active(true);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(false);
            },
            "light" => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(true);
                self.dark_selector.set_active(false);
            },
            "dark" => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(true);
            },
            _ => {
                warn!("Invalid style preference: {:?}", preference)
            },
        }
    }

    #[template_callback]
    fn theme_check_active_changed(&self) {
        if self.system_selector.is_active() {
            self.preference.set("system".to_string());
        } else if self.light_selector.is_active() {
            self.preference.set("light".to_string());
        } else if self.dark_selector.is_active() {
            self.preference.set("dark".to_string());
        }
        self.obj().notify_preference();
    }
}
