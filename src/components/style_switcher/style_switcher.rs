/* StyleSwitcher.vala
 *
 * Copyright 2023 Paulo Queiroz <pvaqueiroz@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use glib::clone;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, CompositeTemplate};
use tracing::*;

use crate::services::settings::{Settings, StylePreference};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/style_switcher.ui")]
pub struct StyleSwitcher {
    settings: Settings,

    #[template_child]
    pub system_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub light_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_selector: TemplateChild<gtk::CheckButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for StyleSwitcher {
    const NAME: &'static str = "TermsStyleSwitcher";
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

impl ObjectImpl for StyleSwitcher {
    fn constructed(&self) {
        self.parent_constructed();

        self.settings
            .connect_style_preference_changed(clone!(@weak self as this => move |_| this.on_style_changed()));
        self.on_style_changed();
    }
}

impl WidgetImpl for StyleSwitcher {}

#[gtk::template_callbacks]
impl StyleSwitcher {
    fn on_style_changed(&self) {
        info!("on style changed");
        let _system_guard = self.system_selector.freeze_notify();
        let _light_guard = self.light_selector.freeze_notify();
        let _dark_guard = self.dark_selector.freeze_notify();

        match self.settings.style_preference() {
            StylePreference::System => {
                self.system_selector.set_active(true);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(false);
            },
            StylePreference::Light => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(true);
                self.dark_selector.set_active(false);
            },
            StylePreference::Dark => {
                self.system_selector.set_active(false);
                self.light_selector.set_active(false);
                self.dark_selector.set_active(true);
            },
        }
    }

    #[template_callback]
    fn theme_check_active_changed(&self) {
        info!("theme_check_active_changed");

        if self.system_selector.is_active() {
            self.change_style_preference(StylePreference::System);
        } else if self.light_selector.is_active() {
            self.change_style_preference(StylePreference::Light);
        } else if self.dark_selector.is_active() {
            self.change_style_preference(StylePreference::Dark);
        }
    }

    fn change_style_preference(&self, style_pref: StylePreference) {
        if self.settings.style_preference() != style_pref {
            info!("Setting style preference {:?}", style_pref);
            self.settings.set_style_preference(style_pref);
        }
    }
}
