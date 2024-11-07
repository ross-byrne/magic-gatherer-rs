use crate::Result;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

/// Using Scryfall API to get magic cards. See documentation here: https://scryfall.com/docs/api

const UNIQUE_ARTWORK_KEY: &'static str = "unique_artwork";
const DEFAULT_CARDS_KEY: &'static str = "default_cards";

pub trait CardApi {
    fn base_url(&self) -> String;
    fn get(&self, url: String) -> RequestBuilder;
}

/// Bulk Data api: https://scryfall.com/docs/api/bulk-data
#[derive(Debug, Deserialize, Serialize)]
pub struct BulkData {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<BulkDataItem>,
}

impl BulkData {
    pub async fn fetch_bulk_data(card_api: &impl CardApi) -> Result<Self> {
        println!("Fetching bulk data from Scryfall API...");

        let bulk_data: BulkData = card_api
            .get(card_api.base_url())
            .send()
            .await?
            .json()
            .await?;

        return Ok(bulk_data);
    }
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
    // pub small: String,
    pub normal: String,
    // pub large: String,
    // pub png: String,
    // pub art_crop: String,
    // pub border_crop: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub image_uris: Option<CardImageUri>,
    //
    // Other fields we aren't using
    //
    // pub object: String,
    // pub oracle_id: String,
    // pub lang: String,
    // pub released_at: String,
    // pub uri: String,
    // pub scryfall_uri: String,
    // pub layout: String,
    // pub highres_image: bool,
    // pub image_status: String,
}

pub enum BulkItemType {
    UniqueArtwork,
    _DefaultCards,
}

impl BulkItemType {
    pub fn get_key(&self) -> &'static str {
        return match self {
            Self::UniqueArtwork => UNIQUE_ARTWORK_KEY,
            Self::_DefaultCards => DEFAULT_CARDS_KEY,
        };
    }

    pub fn get_item<'a>(&self, bulk_data: &'a BulkData) -> &'a BulkDataItem {
        return bulk_data
            .data
            .iter()
            .find(|x| x.item_type == self.get_key())
            .expect("Should find bulk item by type");
    }
}
