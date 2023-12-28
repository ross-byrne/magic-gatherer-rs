use std::env;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";
// const DATA_DIR: &'static str = "/data";
// const CARD_DIR: &'static str = "/data/magic-the-gathering-cards";

fn main() {
    println!("Hello, world!");
    println!("{}", SCRYFALL_API_URL);

    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(_e) => panic!("Error reading current path"),
    };

    println!("The current directory is {}", current_dir.display());
}
