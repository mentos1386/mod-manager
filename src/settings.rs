use gio::glib;
use gsettings_macro::gen_settings;
use gtk::gio;
use tracing::{debug, info};

use crate::config::APP_ID;

#[gen_settings(file = "./data/dev.mnts.ModManager.gschema.xml.in")]
#[gen_settings_define(
    key_name = "games",
    arg_type = "Vec<(String, String)>",
    ret_type = "Vec<(String, String)>"
)]
pub struct Settings;

impl Default for Settings {
    fn default() -> Self {
        Self::new(APP_ID)
    }
}

#[derive(Debug, Clone)]
pub enum Game {
    TheSims4(String),
}
impl Game {
    pub fn from_slug(slug: String, path: String) -> Self {
        match slug.as_str() {
            "sims4" => Game::TheSims4(path),
            _ => panic!("Game not supported"),
        }
    }

    pub fn from_name(name: String, path: String) -> Self {
        match name.as_str() {
            "The Sims 4" => Game::TheSims4(path),
            _ => panic!("Game not supported"),
        }
    }

    pub fn to_path(&self) -> String {
        match self {
            Game::TheSims4(path) => path.to_string(),
        }
    }

    pub fn to_slug(&self) -> String {
        match self {
            Game::TheSims4(_) => "sims4".to_string(),
        }
    }

    pub fn to_name(&self) -> String {
        match self {
            Game::TheSims4(_) => "The Sims 4".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModManagerSettings {
    settings: Settings,
}

impl ModManagerSettings {
    pub fn default() -> Self {
        let settings = Settings::default();
        Self { settings }
    }

    pub fn get_supported_games(&self) -> Vec<Game> {
        info!("Getting supported games");

        return [Game::TheSims4("".to_string())].to_vec();
    }

    pub fn get_managed_games(&self) -> Vec<Game> {
        info!("Getting managed games");

        let mut games_vec: Vec<Game> = Vec::new();

        for (slug, path) in self.settings.games() {
            info!("Found game {:?} at path {:?}", slug, path);
            games_vec.push(Game::from_slug(slug, path));
        }

        return games_vec;
    }

    pub fn add_managed_game(&self, game: Game) {
        info!("Adding game {} {}", game.to_slug(), game.to_name());

        let mut games_vec: Vec<(String, String)> = Vec::new();

        for (slug, path) in self.settings.games() {
            games_vec.push((slug, path));
        }

        games_vec.push((game.to_slug(), game.to_path()));

        self.settings.set_games(games_vec);
    }

    pub fn remove_managed_games(&self) {
        info!("Removing all managed games");

        self.settings.set_games([].to_vec());
    }

    pub fn on_managed_games_changed(&self, callback: impl Fn() + 'static) {
        self.settings.connect_games_changed(move |_| {
            callback();
        });
    }
}
