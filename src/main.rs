use std::fs;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
const DATA_DIR: &'static str = "data";
const CARD_DIR: &'static str = "data/magic-the-gathering-cards";

fn main() {
    println!("Hello, world!");
    println!("{}", SCRYFALL_API_URL);

    create_data_dirs();
}

fn create_data_dirs() {
    match fs::create_dir_all(&DATA_DIR) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }

    match fs::create_dir_all(&CARD_DIR) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
