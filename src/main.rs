use minreq;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    println!("{}", SCRYFALL_API_URL);

    create_data_dirs()?;
    fetch_card_data()?;

    Ok(())
}

fn create_data_dirs() -> Result<(), io::Error> {
    println!("Creating data directories...");

    fs::create_dir_all(&DATA_DIR)?;
    fs::create_dir_all(&CARD_DIR)?;

    Ok(())
}

fn fetch_card_data() -> Result<(), Box<dyn Error>> {
    println!("Querying Scryfall bulk api...");

    let response = minreq::get(SCRYFALL_API_URL).send()?;
    let resp_str = response.as_str()?;

    let bulk_data_json: Value = serde_json::from_str(resp_str)?;
    let data: &Value = &bulk_data_json["data"];

    let default_cards_data: Option<Value> = data
        .as_array()
        .unwrap()
        .iter()
        .find(|&x| x["type"] == "default_cards")
        .cloned();

    if let Some(x) = default_cards_data {
        println!("{}", x);
        Ok(())
    } else {
        panic!("Failed to read API data");
    }
}
