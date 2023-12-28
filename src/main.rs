use serde_json::Value;
use std::fs;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";

fn main() {
    println!("Hello, world!");
    println!("{}", SCRYFALL_API_URL);

    create_data_dirs();
    fetch_card_data();
}

fn create_data_dirs() {
    println!("Creating data directories...");

    match fs::create_dir_all(&DATA_DIR) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }

    match fs::create_dir_all(&CARD_DIR) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}

fn fetch_card_data() {
    println!("Querying Scryfall bulk api...");

    let response_result = minreq::get(SCRYFALL_API_URL).send();
    let response = match response_result {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    println!("{} - {}", response.status_code, response.reason_phrase);

    let resp_str = match response.as_str() {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let bulk_data_json: Value = match serde_json::from_str(resp_str) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let data = &bulk_data_json["data"];
    let mut selected_obj: Option<Value> = None;

    for x in data.as_array().unwrap() {
        // println!("========================================================");
        // println!("{}", x["type"]);

        if x["type"] == "default_cards" {
            selected_obj = Some(x.clone());
            break;
        }
    }

    if let Some(x) = selected_obj {
        println!("{}", x);
    } else {
        panic!("Failed to read API data");
    }
}
