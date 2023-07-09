use bytes::Bytes;

use crate::{
    error::InvalidQueryParameter,
    model::{Character, Server},
    util::AsItem,
    DfClient,
};

#[derive(Clone)]
pub struct ImageHandler {
    client: DfClient,
}

const BASE_URL: &str = "https://img-api.neople.co.kr/df";

impl ImageHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        Self { client }
    }

    pub async fn _character(
        &self,
        server: Server,
        character_id: &str,
        zoom: u8,
    ) -> crate::Result<Bytes> {
        let url = format!("{BASE_URL}/servers/{server}/characters/{character_id}");
        if !(1..=3).contains(&zoom) {
            return Err(InvalidQueryParameter {
                path: url.clone(),
                message: format!("`zoom` must be 1, 2, or 3. (current: `{zoom}`)"),
            }
            .into());
        }

        let response = self
            .client
            .get_with_query(&url, Some(&[("zoom", zoom)]))
            .await?;

        Ok(response.bytes().await?)
    }

    pub async fn character(&self, character: &Character, zoom: u8) -> crate::Result<Bytes> {
        self._character(character.server, &character.id, zoom).await
    }

    pub async fn _item(&self, item_id: &str) -> crate::Result<Bytes> {
        let response = self
            .client
            .get(&format!("{BASE_URL}/items/{item_id}"))
            .await?;

        Ok(response.bytes().await?)
    }

    pub async fn item<T: AsItem>(&self, item: &T) -> crate::Result<Bytes> {
        self._item(item.id()).await
    }
}
