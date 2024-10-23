mod types;

use minreq;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", SCRYFALL_API_URL);

    create_data_dirs()?;
    let _download_uri: String = fetch_card_data()?;

    // download_card_json(&download_uri)?;

    Ok(())
}

fn create_data_dirs() -> Result<(), io::Error> {
    println!("Creating data directories...");

    fs::create_dir_all(&DATA_DIR)?;
    fs::create_dir_all(&CARD_DIR)?;

    Ok(())
}

fn fetch_card_data() -> Result<String, Box<dyn Error>> {
    println!("Querying Scryfall bulk api...");

    let response = minreq::get(SCRYFALL_API_URL).send()?;
    let bulk_data_json: Value = response.json()?;

    let default_cards_data: Option<Value> = bulk_data_json["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|&x| x["type"] == "default_cards")
        .cloned();

    if let Some(data) = default_cards_data {
        let download_uri = data["download_uri"].to_string();

        println!("{}", download_uri);

        Ok(download_uri)
    } else {
        panic!("Failed to read API data");
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();

    chars.as_str()
}

fn download_card_json(url: &String) -> Result<(), Box<dyn Error>> {
    println!("Downloading card json...");

    // remove "" from json string
    let _processed_url = remove_first_and_last(&url);

    // let response = minreq::get(processed_url).send()?;
    // let resp_json: Value = response.json()?;

    // println!("{}", resp_json);

    Ok(())
}
