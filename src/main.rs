mod types;

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
// use serde_json::to_string_pretty;
use std::error::Error;
use std::fs;
use types::{BulkData, BulkDataItem};

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";

const UNIQUE_ARTWORK_KEY: &'static str = "unique_artwork";
const DEFAULT_CARDS_KEY: &'static str = "default_cards";

enum BulkItemType {
    UniqueArtwork,
    DefaultCards,
}

impl BulkItemType {
    pub fn get_key(&self) -> &'static str {
        return match self {
            Self::UniqueArtwork => UNIQUE_ARTWORK_KEY,
            Self::DefaultCards => DEFAULT_CARDS_KEY,
        };
    }

    pub fn get_item<'a>(&self, bulk_data: &'a BulkData) -> &'a BulkDataItem {
        return bulk_data
            .data
            .iter()
            .find(|x| x.item_type == self.get_key())
            .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to magic-gatherer-rs!");
    let client = reqwest::Client::new();

    create_data_dirs();
    let bulk_data = fetch_bulk_data(&client).await?;

    // get unique artwork object
    let unique_artwork: &BulkDataItem = BulkItemType::UniqueArtwork.get_item(&bulk_data);

    println!("{:#?}", unique_artwork);

    // TODO: clean up
    // let _download_uri: String = fetch_card_data()?;
    // download_card_json(&download_uri)?;

    println!("\nFinished! :)\n");
    Ok(())
}

// Recursively create required data directories
fn create_data_dirs() {
    println!("Creating data directories...");
    fs::create_dir_all(&CARD_DIR).expect("Data directories should be created");
}

async fn fetch_bulk_data(client: &reqwest::Client) -> Result<BulkData, Box<dyn Error>> {
    println!("Fetching bulk data from Scryfall API...");

    let bulk_data: BulkData = client
        .get(SCRYFALL_API_URL)
        .headers(get_request_headers())
        .send()
        .await?
        .json()
        .await?;

    // pretty print response for testing
    // let pretty = to_string_pretty(&bulk_data)?;
    println!("{:#?}", bulk_data);

    return Ok(bulk_data);
}

fn get_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Add headers as requested in API docs: https://scryfall.com/docs/api
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MagicGatherer/0.1"));

    return headers;
}

// fn fetch_card_data() -> Result<String, Box<dyn Error>> {
//     println!("Querying Scryfall bulk api...");

//     let response = minreq::get(SCRYFALL_API_URL).send()?;
//     let bulk_data_json: Value = response.json()?;

//     let default_cards_data: Option<Value> = bulk_data_json["data"]
//         .as_array()
//         .unwrap()
//         .iter()
//         .find(|&x| x["type"] == "default_cards")
//         .cloned();

//     if let Some(data) = default_cards_data {
//         let download_uri = data["download_uri"].to_string();

//         println!("{}", download_uri);

//         Ok(download_uri)
//     } else {
//         panic!("Failed to read API data");
//     }
// }

// fn remove_first_and_last(value: &str) -> &str {
//     let mut chars = value.chars();
//     chars.next();
//     chars.next_back();

//     chars.as_str()
// }

// fn download_card_json(url: &String) -> Result<(), Box<dyn Error>> {
//     println!("Downloading card json...");

//     // remove "" from json string
//     let _processed_url = remove_first_and_last(&url);

//     // let response = minreq::get(processed_url).send()?;
//     // let resp_json: Value = response.json()?;

//     // println!("{}", resp_json);

//     Ok(())
// }
