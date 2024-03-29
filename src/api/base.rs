use crate::settings::Game;

pub const API_URL: &str = "https://api.curseforge.com";

pub fn get_game_id(game: Game) -> i32 {
    match game {
        Game::TheSims4(_) => 78062,
        _ => panic!("Game not supported"),
    }
}

pub fn get_curse_forge_client() -> Result<reqwest::blocking::Client, reqwest::Error> {
    let mut api_key_header =
        reqwest::header::HeaderValue::from_str(crate::config::API_KEY).unwrap();
    api_key_header.set_sensitive(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-api-key", api_key_header);

    reqwest::blocking::Client::builder()
        .user_agent(format!("dev.mnts.ModManager/{}", crate::config::VERSION))
        .default_headers(headers)
        .build()
}

pub fn get_async_curse_forge_client() -> Result<reqwest::Client, reqwest::Error> {
    let mut api_key_header =
        reqwest::header::HeaderValue::from_str(crate::config::API_KEY).unwrap();
    api_key_header.set_sensitive(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-api-key", api_key_header);

    reqwest::Client::builder()
        .user_agent(format!("dev.mnts.ModManager/{}", crate::config::VERSION))
        .default_headers(headers)
        .build()
}
