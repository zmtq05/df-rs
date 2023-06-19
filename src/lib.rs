pub mod api;
pub mod error;
use error::ApiError;
pub use error::Error;
use serde::Serialize;
pub mod model;
pub mod util;

use std::sync::OnceLock;

use api::{
    auction::AuctionArtifacts, character::CharacterHandler, image::ImageHandler, item::ItemHandler,
};
use reqwest::Response;

type Result<T, E = Error> = std::result::Result<T, E>;

const DF_BASE_URL: &str = "https://api.neople.co.kr/df";

static STATIC_INSTACE: OnceLock<DfClient> = OnceLock::new();

pub fn instance() -> DfClient {
    STATIC_INSTACE
        .get()
        .expect("DfClient is not initialized")
        .clone()
}

pub fn initialize(api_key: &str) -> DfClient {
    STATIC_INSTACE
        .get_or_init(|| DfClient::new(api_key))
        .clone()
}

#[derive(Clone, Default)]
pub struct DfClient {
    inner: reqwest::Client,
}

impl DfClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", api_key.parse().unwrap());
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self { inner: client }
    }
}

impl DfClient {
    pub fn image(&self) -> ImageHandler {
        ImageHandler::new(self.clone())
    }

    pub fn auction(&self) -> AuctionArtifacts {
        AuctionArtifacts::new(self.clone())
    }

    pub fn character(&self) -> CharacterHandler {
        CharacterHandler::new(self.clone())
    }

    pub fn item(&self) -> ItemHandler {
        ItemHandler::new(self.clone())
    }
}

impl DfClient {
    async fn get(&self, url: &str) -> Result<Response> {
        self.get_with_query::<()>(url, None).await
    }

    async fn get_with_query<T>(&self, url: &str, query: Option<&T>) -> Result<Response>
    where
        T: Serialize + ?Sized,
    {
        let url = if url.starts_with("https://") {
            url.to_owned()
        } else {
            format!("{}{}", DF_BASE_URL, url)
        };
        let response = self.inner.get(url).query(&query).send().await?;

        map_api_error(response).await
    }
}

async fn map_api_error(response: Response) -> Result<Response> {
    if response.status().is_success() {
        return Ok(response);
    }

    Err(Error::Api(ApiError::from_response(response).await))
}
