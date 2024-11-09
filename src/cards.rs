use serde::{Deserialize, Serialize};

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
