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
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::api::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/ui/windows/add_new_game/add_new_game.ui")]
    pub struct ModManagerWindowAddNewGame {
        #[template_child]
        pub complete_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub games_dropdown: TemplateChild<gtk::DropDown>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModManagerWindowAddNewGame {
        const NAME: &'static str = "ModManagerWindowAddNewGame";
        type Type = super::ModManagerWindowAddNewGame;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ModManagerWindowAddNewGame {}
    impl WidgetImpl for ModManagerWindowAddNewGame {}
    impl WindowImpl for ModManagerWindowAddNewGame {}
    impl ApplicationWindowImpl for ModManagerWindowAddNewGame {}
    impl AdwApplicationWindowImpl for ModManagerWindowAddNewGame {}
}

glib::wrapper! {
    pub struct ModManagerWindowAddNewGame(ObjectSubclass<imp::ModManagerWindowAddNewGame>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ModManagerWindowAddNewGame {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn setup(&self) {
        let games = games::get_games();
        let games_strs: Vec<&str> = games.iter().map(|s| s.as_str()).collect();
        let games_list = &gtk::StringList::new(&games_strs);

        let complete_button = &imp::ModManagerWindowAddNewGame::from_obj(self).complete_button;
        let games_dropdown = &imp::ModManagerWindowAddNewGame::from_obj(self).games_dropdown;

        games_dropdown.set_property("model", games_list);

        let instance = self;

        complete_button.connect_clicked(clone!(@strong instance  => move |_| {
            //let selected_game = games_dropdown.selected_item();

            println!("complete button clicked");

            //if let Some(selected_game) = selected_game {
                instance.hide()
            //}
        }));
    }
}
