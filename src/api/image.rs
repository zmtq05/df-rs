use bytes::Bytes;

use crate::{
    model::{Character, Server},
    util::AsItem,
    DfClient,
};

pub struct ImageHandler<'df> {
    client: &'df DfClient,
}

const BASE_URL: &str = "https://img-api.neople.co.kr/df";

impl<'df> ImageHandler<'df> {
    pub(crate) fn new(client: &'df DfClient) -> Self {
        Self { client }
    }

    pub async fn _character(
        &self,
        server: Server,
        character_id: &str,
        zoom: u8,
    ) -> crate::Result<Bytes> {
        let response = self
            .client
            .get(&format!(
                "{BASE_URL}/servers/{server}/characters/{character_id}?zoom={zoom}"
            ))
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
