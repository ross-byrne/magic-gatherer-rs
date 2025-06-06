use crate::Result;
use reqwest::{
    Response,
    header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT},
};

const SCRYFALL_API_URL: &str = "https://api.scryfall.com/bulk-data";

pub trait CardApi {
    fn base_url(&self) -> String;
    async fn get(&self, url: String) -> Result<Response>;
}

pub struct ScryfallApi {
    client: reqwest::Client,
}

impl ScryfallApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
impl CardApi for ScryfallApi {
    fn base_url(&self) -> String {
        SCRYFALL_API_URL.to_string()
    }

    async fn get(&self, url: String) -> Result<Response> {
        let mut headers = HeaderMap::new();

        // Add headers as requested in API docs: https://scryfall.com/docs/api
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("MagicGatherer/0.1"));

        let result = self.client.get(url).headers(headers).send().await;
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scryfall_api_returns_correct_url() {
        let api = ScryfallApi::new();
        let result = api.base_url();
        assert_eq!(result, SCRYFALL_API_URL.to_string());
    }

    #[tokio::test]
    async fn get_requests_have_correct_headers() {
        let api = ScryfallApi::new();

        // Create a mock server
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(201)
            .match_header(ACCEPT, "application/json")
            .match_header(USER_AGENT, "MagicGatherer/0.1")
            .create_async()
            .await;

        // create a test request and fire it
        let test_url = format!("{}/test", server.url());
        let response = api.get(test_url).await;

        // Verify GET request was made with correct headers
        mock.assert_async().await;
        assert!(response.is_ok());
    }
}
