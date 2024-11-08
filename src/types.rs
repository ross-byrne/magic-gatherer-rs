use crate::card_api::CardApi;
use crate::{Result, BULK_DATA_FILE};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::io::AsyncWriteExt;

/// Using Scryfall API to get magic cards. See documentation here: https://scryfall.com/docs/api

const UNIQUE_ARTWORK_KEY: &'static str = "unique_artwork";
const DEFAULT_CARDS_KEY: &'static str = "default_cards";

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
        let bulk_data: BulkData = card_api.get(card_api.base_url()).await?.json().await?;

        return Ok(bulk_data);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkDataItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub download_uri: String,
}

impl BulkDataItem {
    // Downloads BulkDataItem to json file
    pub async fn download_cards_to_file(&self, card_api: &impl CardApi) -> Result<()> {
        // check if file exists and skip download if yes
        if fs::exists(BULK_DATA_FILE)? {
            println!("File for {} already downloaded.", self.name);
            return Ok(());
        }

        println!("Downloading card json for {}...", self.name);

        // Download file and stream response
        let mut stream = card_api
            .get(self.download_uri.to_string())
            .await?
            .bytes_stream();

        // write chunks to file as it downloads
        let mut file = tokio::fs::File::create(BULK_DATA_FILE).await?;
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?).await?;
        }

        return Ok(());
    }
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

/// card api: https://scryfall.com/docs/api/cards/id
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardImageUri {
    pub normal: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardUnprocessed {
    pub id: String,
    pub name: String,
    pub image_uris: Option<CardImageUri>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub image_uri: String,
}

impl From<CardUnprocessed> for Card {
    fn from(unprocessed: CardUnprocessed) -> Self {
        Card {
            id: unprocessed.id,
            name: unprocessed.name,
            image_uri: unprocessed
                .image_uris
                .expect("UnprocessedCard should have image_uris")
                .normal,
        }
    }
}
