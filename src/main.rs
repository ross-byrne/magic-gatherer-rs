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

    println!("{}", resp_str);
}
