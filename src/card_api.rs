use crate::types::CardApi;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    RequestBuilder,
};

const SCRYFALL_API_URL: &'static str = "https://api.scryfall.com/bulk-data";

pub struct ScryfallApi;
impl CardApi for ScryfallApi {
    fn base_url(&self) -> String {
        return SCRYFALL_API_URL.to_string();
    }

    fn get_request(&self, client: &reqwest::Client, url: String) -> RequestBuilder {
        let mut headers = HeaderMap::new();

        // Add headers as requested in API docs: https://scryfall.com/docs/api
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("MagicGatherer/0.1"));

        return client.get(url).headers(headers);
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
