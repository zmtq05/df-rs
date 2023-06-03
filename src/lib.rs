pub mod api;
pub mod error;
pub use error::Error;
pub mod model;

use std::sync::OnceLock;

use api::{auction::AuctionArtifacts, image::ImageHandler};
use reqwest::Response;

type Result<T, E = Error> = std::result::Result<T, E>;

const DF_BASE_URL: &str = "https://api.neople.co.kr/df";

static STATIC_INSTACE: OnceLock<DfClient> = OnceLock::new();

pub fn instance() -> DfClient {
    STATIC_INSTACE.get_or_init(DfClient::default).clone()
}

pub fn initialise(api_key: &str) -> DfClient {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("apikey", api_key.parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    STATIC_INSTACE
        .set(DfClient { inner: client })
        .unwrap_or_else(|_| panic!("unable to initialise DfClient"));
    STATIC_INSTACE.get().unwrap().clone()
}

#[derive(Clone, Default)]
pub struct DfClient {
    inner: reqwest::Client,
}

impl DfClient {
    pub fn image(&self) -> ImageHandler<'_> {
        ImageHandler::new(self)
    }

    pub fn auction(&self) -> AuctionArtifacts<'_> {
        AuctionArtifacts::new(self)
    }
}

impl DfClient {
    async fn get(&self, url: &str) -> Result<Response> {
        let response = self.inner.get(url).send().await?;

        map_api_error(response).await
    }
}

async fn map_api_error(response: Response) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        Err(Error::Api(response.json().await?))
    }
}
