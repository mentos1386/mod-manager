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

use crate::windows::ModManagerWindowAddNewGame;

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/ui/windows/main/pages/welcome.ui")]
    pub struct Welcome {
        #[template_child]
        pub add_new_game: gtk::TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Welcome {
        const NAME: &'static str = "Welcome";
        type Type = super::Welcome;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Welcome {
        fn constructed(&self) {
            self.parent_constructed();

            self.add_new_game.connect_clicked(|_| {
                let welcome = ModManagerWindowAddNewGame::new();
                welcome.setup();
                welcome.set_modal(true);
                welcome.present();
            });
        }
    }
    impl WidgetImpl for Welcome {}
    impl BinImpl for Welcome {}
}

glib::wrapper! {
    pub struct Welcome(ObjectSubclass<imp::Welcome>)
        @extends gtk::Widget, adw::ToolbarView,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Welcome {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
