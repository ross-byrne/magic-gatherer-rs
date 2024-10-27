mod types;

use futures_util::StreamExt;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
// use serde_json::to_string_pretty;
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use types::{BulkData, BulkDataItem, BulkItemType, Card};

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";
const BULK_DATA_FILE: &'static str = "bulk-data.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to magic-gatherer-rs!");
    let client = reqwest::Client::new();

    create_data_dirs();
    let bulk_data = fetch_bulk_data(&client).await?;

    // get unique artwork object
    let unique_artwork: &BulkDataItem = BulkItemType::UniqueArtwork.get_item(&bulk_data);
    // println!("{:#?}", unique_artwork);

    // start downloading card json file
    download_card_json(&client, &unique_artwork.download_uri).await?;

    // parse downloaded file for card IDs and download URIs
    let cards = parse_card_json_file()?;

    println!("number of parsed cards: {}", cards.len());
    println!("First card: {:#?}", cards[0]);

    println!("\nFinished!\n");
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
    // println!("{:#?}", bulk_data);

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
    // define file path
    let mut file_path = Path::new(DATA_DIR).to_path_buf();
    file_path.push(BULK_DATA_FILE);

    // check if file exists and skip download if yes
    // TODO: check expected file size from BulkDataItem. Remove file and download again if it doesn't match
    if fs::exists(&file_path)? {
        println!("File already downloaded.");
        return Ok(());
    }

    println!("Downloading card json...");

    // stream response
    let mut stream = client
        .get(download_uri)
        .headers(get_request_headers())
        .send()
        .await?
        .bytes_stream();

    // write chunks to file as it downloads
    let mut file = tokio::fs::File::create(file_path).await?;
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    return Ok(());
}

fn parse_card_json_file() -> Result<Vec<Card>, Box<dyn Error>> {
    println!("Parsing downloaded json file...");
    let mut path = Path::new(DATA_DIR).to_path_buf();
    path.push(BULK_DATA_FILE);

    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("File should be opened as read only");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let cards: Vec<Card> = serde_json::from_reader(reader)?;

    // Filter out entries missing image uris
    let cards: Vec<Card> = cards
        .into_iter()
        .filter(|x| x.image_uris.is_some())
        .collect();

    return Ok(cards);
}
