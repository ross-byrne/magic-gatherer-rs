pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

mod bulk_data;
mod card_api;
mod cards;

use bulk_data::{BulkData, BulkDataItem, BulkItemType};
use card_api::{CardApi, ScryfallApi};
use cards::{Card, CardUnprocessed};
use futures_util::StreamExt;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use tokio::io::AsyncWriteExt;

const DATA_DIR: &str = "data";
const CARD_DIR: &str = "data/magic-the-gathering-cards";
const BULK_DATA_FILE: &str = "data/bulk-data.json";
const PROCESSED_CARD_DATA_FILE: &str = "data/processed-card-data.json";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to magic-gatherer-rs!");

    // create new instance of scryfall api
    let scryfall_api = ScryfallApi::new();
    let cards: Vec<Card>;

    // setup data directory
    create_data_dirs();

    // if processed card data doesn't exist yet
    if !fs::exists(PROCESSED_CARD_DATA_FILE)? {
        // fetch bulk data
        let bulk_data = BulkData::fetch_bulk_data(&scryfall_api).await?;

        // get unique artwork object and download cards json
        let unique_artwork: &BulkDataItem = BulkItemType::UniqueArtwork.get_item(&bulk_data);
        unique_artwork.download_cards_to_file(&scryfall_api).await?;

        // parse downloaded file for card IDs and download URIs
        cards = parse_card_json_file()?;
        save_processed_json_to_file(&cards)?;
    } else {
        println!("Processed card data already exists...");

        // read file to get cards
        cards = parse_processed_card_json_file()?;
    }

    println!("Number of parsed cards: {}", cards.len());

    // start downloading images
    download_card_images(&scryfall_api, cards).await?;

    println!("\nFinished!\n");
    Ok(())
}

// Recursively create required data directories
fn create_data_dirs() {
    println!("Creating data directories...");
    fs::create_dir_all(DATA_DIR).expect("Data directory should be created");
    fs::create_dir_all(CARD_DIR).expect("Card directory should be created");
}

// Parses the bulk card json file
fn parse_card_json_file() -> Result<Vec<Card>> {
    println!("Parsing downloaded json file...");

    // Open the file in read-only mode with buffer.
    let file = fs::File::open(BULK_DATA_FILE).expect("File should be opened as read only");
    let reader = std::io::BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let cards_unprocessed: Vec<CardUnprocessed> = serde_json::from_reader(reader)?;

    // Filter out entries missing image uris
    let cards_unprocessed: Vec<CardUnprocessed> = cards_unprocessed
        .into_iter()
        .filter(|x| x.image_uris.is_some())
        .collect();

    // Process card json to make it less nested
    let cards: Vec<Card> = cards_unprocessed
        .into_iter()
        .map(Card::from)
        .collect();

    Ok(cards)
}

// Saves processed card json to file
fn save_processed_json_to_file(data: &Vec<Card>) -> Result<()> {
    if fs::exists(PROCESSED_CARD_DATA_FILE)? {
        println!("Processed file already created...");
        return Ok(());
    }

    // create file to save processed card data
    let mut output = fs::File::create(PROCESSED_CARD_DATA_FILE)?;

    // serialse structs as json and write it to the file
    let json = serde_json::to_string(&data).expect("Struct should be serialised");
    output.write_all(json.as_bytes())?;

    Ok(())
}

// Parses the processed card data, for download
fn parse_processed_card_json_file() -> Result<Vec<Card>> {
    if !fs::exists(PROCESSED_CARD_DATA_FILE)? {
        return Err("Processed card json file does not exist".into());
    }

    println!("Reading processed card json from file...");

    // Open the file in read-only mode with buffer.
    let file =
        fs::File::open(PROCESSED_CARD_DATA_FILE).expect("File should be opened as read only");
    let reader = std::io::BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let cards: Vec<Card> = serde_json::from_reader(reader)?;

    Ok(cards)
}

// Downloads a card image
async fn download_card_image(
    card_api: &impl CardApi,
    card: &Card,
    count: usize,
    total: usize,
) -> Result<()> {
    // check if file exists and skip download if yes
    // TODO: check expected file size. Remove file and download again if it doesn't match
    let file_path: String = format!("{}/{}.png", CARD_DIR, card.id);
    if fs::exists(&file_path)? {
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

    // fetch card image and stream response
    let mut stream = card_api
        .get(card.image_uri.to_owned())
        .await?
        .bytes_stream();

    // write chunks to file as it downloads
    let mut file = tokio::fs::File::create(file_path).await?;
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    // rate limit requests to follow api rules. See: https://scryfall.com/docs/api
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    Ok(())
}

// Handles downloading all cards
async fn download_card_images(card_api: &impl CardApi, cards: Vec<Card>) -> Result<()> {
    println!("\nStarting image download...\n");

    let iter = cards.iter();
    let mut count: usize = 0;
    let total = cards.len();

    // download each card image if not already downloaded
    for card in iter {
        count += 1;
        download_card_image(card_api, card, count, total).await?;
    }

    Ok(())
}
