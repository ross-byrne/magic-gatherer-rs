use serde::{Deserialize, Serialize};

/// Using Scryfall API to get magic cards. See documentation here: https://scryfall.com/docs/api

/// Bulk Data api: https://scryfall.com/docs/api/bulk-data
#[derive(Debug, Deserialize, Serialize)]
pub struct BulkData {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<BulkDataItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkDataItem {
    pub object: String,
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub updated_at: String,
    pub uri: String,
    pub download_uri: String,
    pub size: u32,
    pub name: String,
    pub description: String,
}

/// card api: https://scryfall.com/docs/api/cards/id
#[derive(Debug, Deserialize, Serialize)]
pub struct CardImageUri {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: String,
    pub highres_image: bool,
    pub image_status: String,
    pub image_uris: CardImageUri,
}
