use std::fmt::Display;

use bytes::Bytes;
use itertools::join;
use serde::Serialize;

use crate::{
    model::{ItemInfo, ItemRarity, SearchItem},
    DfClient, Result,
};

use super::WordType;

pub struct ItemHandler {
    client: DfClient,

    item_name: String,
    item_id: String,

    param: Param,
}

impl ItemHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        Self {
            client,
            item_id: Default::default(),
            item_name: Default::default(),
            param: Default::default(),
        }
    }
}

impl ItemHandler {
    pub async fn search(&self) -> Result<Vec<SearchItem>> {
        let resp = self
            .client
            .get_with_query(
                &format!("/items?itemName={name}", name = self.item_name),
                Some(&self.param),
            )
            .await?;

        Ok(unwrap_rows!(resp, SearchItem))
    }

    pub async fn info(&self) -> Result<ItemInfo> {
        let resp = self
            .client
            .get(&format!("/items/{id}", id = self.item_id))
            .await?;

        Ok(resp.json().await.unwrap())
    }

    pub async fn multi_info(&self) -> Result<Vec<ItemInfo>> {
        let resp = self
            .client
            .get(&format!("/multi/items?itemIds={id}", id = self.item_id))
            .await?;

        Ok(unwrap_rows!(resp, ItemInfo))
    }

    pub async fn image(&self, item_id: &str) -> Result<Bytes> {
        self.client.image()._item(item_id).await
    }
}

impl ItemHandler {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.item_name = name.into();
        self
    }
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.item_id = id.into();
        self
    }
    pub fn id_iter<I>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        self.item_id = join(ids, ",");
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
        self.param.q.max_level = Some(max_level);
        self
    }
    pub fn min_level(&mut self, min_level: u8) -> &mut Self {
        self.param.q.min_level = Some(min_level);
        self
    }
    pub fn rarity(&mut self, rarity: ItemRarity) -> &mut Self {
        self.param.q.rarity = Some(rarity);
        self
    }
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    limit: Option<u8>,
    word_type: Option<WordType>,
    q: Query,
}

#[derive(Default, Clone)]
pub struct Query {
    min_level: Option<u8>,
    max_level: Option<u8>,
    rarity: Option<ItemRarity>,
}

nested_query!(Query; min_level, max_level, rarity);
