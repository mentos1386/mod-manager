/* welcome.rs
 *
 * Copyright 2023 Tine
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

use adw::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{glib::clone, prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/components/mods_list.ui")]
    pub struct ModsList {
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModsList {
        const NAME: &'static str = "ModsList";
        type Type = super::ModsList;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ModsList {
        fn constructed(&self) {
        }
    }
    impl WidgetImpl for ModsList {}
    impl NavigationPageImpl for ModsList {}
}

glib::wrapper! {
    pub struct ModsList(ObjectSubclass<imp::ModsList>)
        @extends gtk::Widget, gtk::Buildable, adw::NavigationPage,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ModsList {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
