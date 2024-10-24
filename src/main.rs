mod types;

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde_json::to_string_pretty;
use std::error::Error;
use std::fs;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use types::{BulkData, BulkDataItem, Card};

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";
const BULK_DATA_FILE: &'static str = "bulk-data.json";

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
            .expect("Should find bulk item by type");
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

    // start downloading card json file
    download_card_json(&client, &unique_artwork.download_uri).await?;

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

async fn download_card_json(
    client: &reqwest::Client,
    download_uri: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Downloading card json...");

    // get response
    let mut response = client
        .get(download_uri)
        .headers(get_request_headers())
        .send()
        .await?;

    // define file path. TODO: add card set type to file name?
    let mut file_path = Path::new(DATA_DIR).to_path_buf();
    file_path.push(BULK_DATA_FILE);

    // write chunks to file as it downloads
    let mut file = tokio::fs::File::create(file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    // pretty print response for testing
    // let pretty = to_string_pretty(&card_json).unwrap();
    // println!("{}", pretty);

    return Ok(());
}
