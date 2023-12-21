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
use gtk::glib::subclass::types::FromObject;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::api::*;
use crate::settings::{Game, ModManagerSettings};

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

    pub fn show() -> Self {
        let welcome = ModManagerWindowAddNewGame::new();
        welcome.setup();
        welcome.set_modal(true);
        welcome.present();

        return welcome;
    }

    pub fn setup(&self) {
        let settings = ModManagerSettings::default();

        let supported_games = settings.get_supported_games();
        let games_strs: Vec<String> = supported_games.iter().map(|s| s.to_name()).collect();
        let games_list =
            &gtk::StringList::new(&games_strs.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

        let obj = self.imp();

        obj.games_dropdown.set_property("model", games_list);

        let instance = self;

        obj.complete_button
            .connect_clicked(clone!(@strong instance, @strong games_list, @strong settings  => move |_| {
                //let selected_game = games_dropdown.selected_item();

                let game_selected = games_list.string(instance.imp().games_dropdown.selected()).unwrap().to_string();
                println!("complete button clicked, selected game: {:?}", &game_selected);

                settings.add_managed_game(Game::from_name(game_selected, "".to_string()));

                //if let Some(selected_game) = selected_game {
                    instance.hide()
                //}
            }));
    }
}
