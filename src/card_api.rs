use crate::types::*;

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";

pub struct ScryfallApi;
impl CardApi for ScryfallApi {
    fn base_url(&self) -> String {
        return SCRYFALL_API_URL.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scryfall_api_returns_correct_url() {
        let result = ScryfallApi.base_url();
        assert_eq!(result, SCRYFALL_API_URL.to_string());
    }
}
