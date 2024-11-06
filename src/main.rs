pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

mod types;

use futures_util::StreamExt;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
// use serde_json::to_string_pretty;
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use tokio::io::AsyncWriteExt;
use types::{BulkData, BulkDataItem, BulkItemType, Card, CardImageUri};

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";
const BULK_DATA_FILE: &'static str = "data/bulk-data.json";
const PROCESSED_CARD_DATA_FILE: &'static str = "data/processed-card-data.json";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to magic-gatherer-rs!");
    let client = reqwest::Client::new();

    // setup data directory
    create_data_dirs();

    let cards: Vec<Card>;

    // if processed card data doesn't exist yet
    if !fs::exists(PROCESSED_CARD_DATA_FILE)? {
        // fetch bulk data
        let bulk_data = fetch_bulk_data(&client).await?;

        // get unique artwork object
        let unique_artwork: &BulkDataItem = BulkItemType::UniqueArtwork.get_item(&bulk_data);
        // println!("{:#?}", unique_artwork);

        // start downloading card json file
        download_card_json(&client, &unique_artwork.download_uri).await?;

        // parse downloaded file for card IDs and download URIs
        cards = parse_card_json_file()?;

        // save processed card data to a file
        save_processed_json_to_file(&cards)?;
    } else {
        println!("Processed card data already exists...");

        // read file to get cards
        cards = parse_processed_card_json_file()?;
    }

    println!("number of parsed cards: {}", cards.len());
    // println!("First card: {:#?}", cards[0]);

    // start downloading images
    download_card_images(&client, cards).await?;

    println!("\nFinished!\n");
    Ok(())
}

// Recursively create required data directories
fn create_data_dirs() {
    println!("Creating data directories...");
    fs::create_dir_all(&DATA_DIR).expect("Data directory should be created");
    fs::create_dir_all(&CARD_DIR).expect("Card directory should be created");
}

async fn fetch_bulk_data(client: &reqwest::Client) -> Result<BulkData> {
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

async fn download_card_json(client: &reqwest::Client, download_uri: &str) -> Result<()> {
    // check if file exists and skip download if yes
    // TODO: check expected file size from BulkDataItem. Remove file and download again if it doesn't match
    if fs::exists(BULK_DATA_FILE)? {
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
    let mut file = tokio::fs::File::create(BULK_DATA_FILE).await?;
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    return Ok(());
}

fn parse_card_json_file() -> Result<Vec<Card>> {
    println!("Parsing downloaded json file...");

    // Open the file in read-only mode with buffer.
    let file = File::open(BULK_DATA_FILE).expect("File should be opened as read only");
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

fn save_processed_json_to_file(data: &Vec<Card>) -> Result<()> {
    if fs::exists(PROCESSED_CARD_DATA_FILE)? {
        println!("Processed file already created...");
        return Ok(());
    }

    // create file to save processed card data
    let mut output = File::create(PROCESSED_CARD_DATA_FILE)?;

    // serialse structs as json and write it to the file
    let json = serde_json::to_string(&data).expect("Struct should be serialised");
    output.write_all(json.as_bytes())?;

    Ok(())
}

fn parse_processed_card_json_file() -> Result<Vec<Card>> {
    if !fs::exists(PROCESSED_CARD_DATA_FILE)? {
        return Err("Processed card json file does not exist".into());
    }

    println!("Reading processed card json from file...");

    // Open the file in read-only mode with buffer.
    let file = File::open(PROCESSED_CARD_DATA_FILE).expect("File should be opened as read only");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let cards: Vec<Card> = serde_json::from_reader(reader)?;

    return Ok(cards);
}

async fn download_card_image(
    client: &reqwest::Client,
    card: &Card,
    count: usize,
    total: usize,
) -> Result<()> {
    // check if file exists and skip download if yes
    // TODO: check expected file size. Remove file and download again if it doesn't match
    let file_path: String = format!("{}/{}.png", CARD_DIR, card.id);
    if fs::exists(file_path.to_owned())? {
        println!(
            "{}/{}: Card already downloaded. Name: \"{}\", ID: {}...",
            count, total, card.name, card.id
        );
        return Ok(());
    }

    println!(
        "{}/{}: Downloading Card Name: \"{}\", ID: {}...",
        count, total, card.name, card.id
    );

    // get download uri from card
    let image_uris: &CardImageUri = card
        .image_uris
        .as_ref()
        .expect("Card should have image uris");
    let download_uri: String = image_uris.normal.to_owned();

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

    // rate limit requests to follow api rules. See: https://scryfall.com/docs/api
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    return Ok(());
}

async fn download_card_images(client: &reqwest::Client, cards: Vec<Card>) -> Result<()> {
    println!("\nStarting image download...\n");

    let mut iter = cards.iter();
    let mut count: usize = 0;
    let total = cards.len();

    // download each card image if not already downloaded
    while let Some(card) = iter.next() {
        count += 1;
        download_card_image(&client, &card, count, total).await?;
    }

    return Ok(());
}
