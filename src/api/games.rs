use serde::Deserialize;

use crate::api::*;

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Game>,
    pagination: Pagination,
}

#[derive(Deserialize, Debug)]
struct Game {
    id: u32,
    name: String,
    slug: String,
    dateModified: String,
    assets: Assets,
    status: u32,
    apiStatus: u32,
}

#[derive(Deserialize, Debug)]
struct Assets {
    iconUrl: Option<String>,
    tileUrl: String,
    coverUrl: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Pagination {
    index: u32,
    pageSize: u32,
    resultCount: u32,
    totalCount: u32,
}

pub fn get_games() -> Vec<String> {
    let client = base::get_curse_forge_client().unwrap();
    let response = client
        .get(&format!("{}/v1/games", base::API_URL))
        .send()
        .unwrap();
    let json: Response = response.json().unwrap();

    return json.data.iter().map(|game| game.name.clone()).collect();
}
