use std::fmt::Display;

use bytes::Bytes;
use itertools::join;
use serde::Serialize;
use urlencoding::encode;

use crate::{
    model::{ItemInfo, ItemRarity, SearchItem},
    DfClient, Result,
};

use super::WordType;

#[derive(Clone)]
pub struct ItemHandler {
    client: DfClient,
    param: ItemSearchParameter,
}

/// # Constructor
impl ItemHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        Self {
            client,
            param: Default::default(),
        }
    }
}

/// # Send Request
impl ItemHandler {
    pub async fn search(&self) -> Result<Vec<SearchItem>> {
        let resp = self
            .client
            .get_with_query(
                &format!(
                    "/items?itemName={name}",
                    name = encode(&self.param.item_name)
                ),
                Some(&self.param),
            )
            .await?;

        Ok(unwrap_rows!(resp, SearchItem))
    }

    pub async fn info(&self) -> Result<ItemInfo> {
        let resp = self
            .client
            .get(&format!("/items/{id}", id = self.param.item_id))
            .await?;

        Ok(resp.json().await.unwrap())
    }

    pub async fn multi_info(&self) -> Result<Vec<ItemInfo>> {
        let resp = self
            .client
            .get(&format!(
                "/multi/items?itemIds={id}",
                id = self.param.item_id
            ))
            .await?;

        Ok(unwrap_rows!(resp, ItemInfo))
    }

    pub async fn image(&self, item_id: &str) -> Result<Bytes> {
        self.client.image()._item(item_id).await
    }
}

/// # Parameter
impl ItemHandler {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.param.item_name = name.into();
        self
    }
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.param.item_id = id.into();
        self
    }
    pub fn id_iter<I>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        self.param.item_id = join(ids, ",");
        self
    }
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.param.limit = Some(limit);
        self
    }
    pub fn word_type(&mut self, word_type: WordType) -> &mut Self {
        self.param.word_type = Some(word_type);
        self
    }
    pub fn max_level(&mut self, max_level: u8) -> &mut Self {
        self.param.query.max_level = Some(max_level);
        self
    }
    pub fn min_level(&mut self, min_level: u8) -> &mut Self {
        self.param.query.min_level = Some(min_level);
        self
    }
    pub fn rarity(&mut self, rarity: ItemRarity) -> &mut Self {
        self.param.query.rarity = Some(rarity);
        self
    }
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSearchParameter {
    #[serde(skip)]
    pub item_name: String,
    #[serde(skip)]
    pub item_id: String,
    pub limit: Option<u8>,
    pub word_type: Option<WordType>,
    #[serde(rename = "q")]
    pub query: Query,
}

#[derive(Default, Clone)]
pub struct Query {
    pub min_level: Option<u8>,
    pub max_level: Option<u8>,
    pub rarity: Option<ItemRarity>,
}

nested_query!(Query; min_level, max_level, rarity);
