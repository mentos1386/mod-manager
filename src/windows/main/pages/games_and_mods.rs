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

use adw::prelude::PreferencesRowExt;
use adw::subclass::prelude::*;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use tracing::info;

use crate::{
    api::{get_game_id, get_mods},
    components::Card,
    dispatch::spawn_task_handler,
    settings::{Game, ModManagerSettings},
    windows::ModManagerWindowAddNewGame,
};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/ui/windows/main/pages/games_and_mods.ui")]
    pub struct GamesAndMods {
        #[template_child]
        pub add_new_game: TemplateChild<gtk::Button>,

        #[template_child]
        pub remove_all_games: TemplateChild<gtk::Button>,

        #[template_child]
        pub games_list: TemplateChild<gtk::ListBox>,

        #[template_child]
        pub mods_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GamesAndMods {
        const NAME: &'static str = "GamesAndMods";
        type Type = super::GamesAndMods;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GamesAndMods {
        fn constructed(&self) {
            let context = glib::MainContext::default();
            let worker = spawn_task_handler(&context);

            self.parent_constructed();
            let obj = self.obj();

            self.remove_all_games.connect_clicked(|_| {
                let settings = ModManagerSettings::default();
                settings.remove_managed_games();
                println!("Remove all games button clicked");
            });

            self.add_new_game.connect_clicked(|_| {
                ModManagerWindowAddNewGame::show();
            });

            let settings = ModManagerSettings::default();

            for game in settings.get_managed_games() {
                info!("Adding game {:?} to list", game);
                let row = adw::ActionRow::new();
                row.set_title(&game.to_name());
                obj.imp().games_list.append(&row);
            }

            settings.on_managed_games_changed(clone!(@weak obj, @strong settings => move || {
                info!("Games changed, modifying list");
                for game in settings.get_managed_games() {
                    info!("Adding game {:?} to list", game);
                    let row = adw::ActionRow::new();
                    row.set_title(&game.to_name());
                    obj.imp().games_list.append(&row);
                }
            }));

            let mods = get_mods(&get_game_id(Game::TheSims4("".to_string())))
                .iter()
                .for_each(|mod_| {
                    let card = Card::new();
                    card.bind(worker.clone(), &mod_.name, &mod_.summary, &mod_.logo.url);

                    obj.imp().mods_list.append(&card);
                });
        }
    }
    impl WidgetImpl for GamesAndMods {}
    impl BinImpl for GamesAndMods {}
}

glib::wrapper! {
    pub struct GamesAndMods(ObjectSubclass<imp::GamesAndMods>)
        @extends gtk::Widget, adw::NavigationSplitView,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GamesAndMods {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
