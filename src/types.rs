use serde::{Deserialize, Serialize};

/// Using Scryfall API to get magic cards. See documentation here: https://scryfall.com/docs/api

/// Bulk Data api: https://scryfall.com/docs/api/bulk-data
#[derive(Debug, Deserialize, Serialize)]
pub struct BulkData {
    object: String,
    has_more: String,
    data: Vec<BulkDataItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkDataItem {
    object: String,
    id: String,
    #[serde(rename = "type")]
    item_type: String,
    updated_at: String,
    uri: String,
    download_uri: String,
    name: String,
    description: String,
}

/// card api: https://scryfall.com/docs/api/cards/id
#[derive(Debug, Deserialize, Serialize)]
pub struct CardImageUri {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    object: String,
    id: String,
    oracle_id: String,
    name: String,
    lang: String,
    released_at: String,
    uri: String,
    scryfall_uri: String,
    layout: String,
    highres_image: bool,
    image_status: String,
    image_uris: CardImageUri,
}
