use serde::Deserialize;

use super::{get_curse_forge_client, API_URL};

/*
{
  "data": [
    {
      "id": 0,
      "gameId": 0,
      "name": "string",
      "slug": "string",
      "links": {
        "websiteUrl": "string",
        "wikiUrl": "string",
        "issuesUrl": "string",
        "sourceUrl": "string"
      },
      "summary": "string",
      "status": 1,
      "downloadCount": 0,
      "isFeatured": true,
      "primaryCategoryId": 0,
      "categories": [
        {
          "id": 0,
          "gameId": 0,
          "name": "string",
          "slug": "string",
          "url": "string",
          "iconUrl": "string",
          "dateModified": "2019-08-24T14:15:22Z",
          "isClass": true,
          "classId": 0,
          "parentCategoryId": 0,
          "displayIndex": 0
        }
      ],
      "classId": 0,
      "authors": [
        {
          "id": 0,
          "name": "string",
          "url": "string"
        }
      ],
      "logo": {
        "id": 0,
        "modId": 0,
        "title": "string",
        "description": "string",
        "thumbnailUrl": "string",
        "url": "string"
      },
      "screenshots": [
        {
          "id": 0,
          "modId": 0,
          "title": "string",
          "description": "string",
          "thumbnailUrl": "string",
          "url": "string"
        }
      ],
      "mainFileId": 0,
      "latestFiles": [
        {
          "id": 0,
          "gameId": 0,
          "modId": 0,
          "isAvailable": true,
          "displayName": "string",
          "fileName": "string",
          "releaseType": 1,
          "fileStatus": 1,
          "hashes": [
            {
              "value": "string",
              "algo": 1
            }
          ],
          "fileDate": "2019-08-24T14:15:22Z",
          "fileLength": 0,
          "downloadCount": 0,
          "fileSizeOnDisk": 0,
          "downloadUrl": "string",
          "gameVersions": [
            "string"
          ],
          "sortableGameVersions": [
            {
              "gameVersionName": "string",
              "gameVersionPadded": "string",
              "gameVersion": "string",
              "gameVersionReleaseDate": "2019-08-24T14:15:22Z",
              "gameVersionTypeId": 0
            }
          ],
          "dependencies": [
            {
              "modId": 0,
              "relationType": 1
            }
          ],
          "exposeAsAlternative": true,
          "parentProjectFileId": 0,
          "alternateFileId": 0,
          "isServerPack": true,
          "serverPackFileId": 0,
          "isEarlyAccessContent": true,
          "earlyAccessEndDate": "2019-08-24T14:15:22Z",
          "fileFingerprint": 0,
          "modules": [
            {
              "name": "string",
              "fingerprint": 0
            }
          ]
        }
      ],
      "latestFilesIndexes": [
        {
          "gameVersion": "string",
          "fileId": 0,
          "filename": "string",
          "releaseType": 1,
          "gameVersionTypeId": 0,
          "modLoader": 0
        }
      ],
      "latestEarlyAccessFilesIndexes": [
        {
          "gameVersion": "string",
          "fileId": 0,
          "filename": "string",
          "releaseType": 1,
          "gameVersionTypeId": 0,
          "modLoader": 0
        }
      ],
      "dateCreated": "2019-08-24T14:15:22Z",
      "dateModified": "2019-08-24T14:15:22Z",
      "dateReleased": "2019-08-24T14:15:22Z",
      "allowModDistribution": true,
      "gamePopularityRank": 0,
      "isAvailable": true,
      "thumbsUpCount": 0
    }
  ],
  "pagination": {
    "index": 0,
    "pageSize": 0,
    "resultCount": 0,
    "totalCount": 0
  }
}
*/
#[derive(Debug, Deserialize)]
struct Hash {
    value: String,
    algo: i32,
}

#[derive(Debug, Deserialize)]
struct SortableGameVersion {
    gameVersionName: String,
    gameVersionPadded: String,
    gameVersion: String,
    gameVersionReleaseDate: String,
    gameVersionTypeId: i32,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    modId: i32,
    relationType: i32,
}

#[derive(Debug, Deserialize)]
struct Module {
    name: String,
    fingerprint: i32,
}

#[derive(Debug, Deserialize)]
pub struct Mod {
    pub name: String,
    //pub modId: i32,
    //pub gameSlug: String,
    //pub gameId: i32,
    pub summary: String,
    //pub defaultFileId: i32,
    //pub downloadCount: i32,
    //pub latestFiles: Vec<LatestFile>,
    //pub latestFilesIndexes: Vec<LatestFileIndex>,
    //pub latestEarlyAccessFilesIndexes: Vec<LatestEarlyAccessFileIndex>,
    //pub dateCreated: String,
    //pub dateModified: String,
    //pub dateReleased: String,
    //pub allowModDistribution: bool,
    //pub gamePopularityRank: i32,
    //pub isAvailable: bool,
    //pub thumbsUpCount: i32,
}

#[derive(Debug, Deserialize)]
struct LatestFile {
    fileId: i32,
    displayName: String,
    fileName: String,
    fileDate: String,
    fileLength: i32,
    releaseType: i32,
    fileStatus: i32,
    downloadUrl: String,
    gameVersions: Vec<String>,
    sortableGameVersions: Vec<SortableGameVersion>,
    dependencies: Vec<Dependency>,
    exposeAsAlternative: bool,
    parentProjectFileId: i32,
    alternateFileId: i32,
    isServerPack: bool,
    serverPackFileId: i32,
    isEarlyAccessContent: bool,
    earlyAccessEndDate: String,
    fileFingerprint: i32,
    modules: Vec<Module>,
}

#[derive(Debug, Deserialize)]
struct LatestFileIndex {
    gameVersion: String,
    fileId: i32,
    filename: String,
    releaseType: i32,
    gameVersionTypeId: i32,
    modLoader: i32,
}

#[derive(Debug, Deserialize)]
struct LatestEarlyAccessFileIndex {
    gameVersion: String,
    fileId: i32,
    filename: String,
    releaseType: i32,
    gameVersionTypeId: i32,
    modLoader: i32,
}

#[derive(Debug, Deserialize)]
struct GetModsResponse {
    data: Vec<Mod>,
    pagination: Pagination,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    index: i32,
    pageSize: i32,
    resultCount: i32,
    totalCount: i32,
}

pub fn get_mods(game_id: &i32) -> Vec<Mod> {
    let client = get_curse_forge_client().unwrap();

    let response = client
        .get(&format!("{}/v1/mods/search", API_URL))
        .query(&[("gameId", game_id)])
        .send()
        .unwrap();
    let json: GetModsResponse = response.json().unwrap();

    return json.data;
}
