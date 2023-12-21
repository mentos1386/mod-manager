/*
Copyright (c) 2023 Tine Jozelj

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use tracing::{debug, info};

use crate::config::PROFILE;
use crate::settings::ModManagerSettings;
use crate::windows::GamesAndMods;

use super::Welcome;

mod imp {

    use gtk::glib::clone;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/ui/windows/main/main.ui")]
    pub struct ModManagerWindowMain {}

    #[glib::object_subclass]
    impl ObjectSubclass for ModManagerWindowMain {
        const NAME: &'static str = "ModManagerWindowMain";
        type Type = super::ModManagerWindowMain;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ModManagerWindowMain {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            let settings = ModManagerSettings::default();
            settings.connect_games_changed(clone!(@strong obj, @strong settings => move |_| {
                info!("Games changed, deciding on initial page.");

                let page: gtk::Widget = if settings.games().len() > 0 {
                    GamesAndMods::new().upcast()
                } else {
                    Welcome::new().upcast()
                };

                obj.set_property("content", page);
            }));

            let page: gtk::Widget = if settings.games().len() > 0 {
                GamesAndMods::new().upcast()
            } else {
                Welcome::new().upcast()
            };

            obj.set_property("content", page)
        }
    }

    impl WidgetImpl for ModManagerWindowMain {}
    impl WindowImpl for ModManagerWindowMain {}
    impl ApplicationWindowImpl for ModManagerWindowMain {}
    impl AdwApplicationWindowImpl for ModManagerWindowMain {}
}

glib::wrapper! {
    pub struct ModManagerWindowMain(ObjectSubclass<imp::ModManagerWindowMain>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ModManagerWindowMain {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
