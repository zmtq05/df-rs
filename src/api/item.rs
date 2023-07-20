use std::fmt::Display;

use bytes::Bytes;
use itertools::join;
use serde::Serialize;
use urlencoding::encode;

use crate::{
    error::InvalidQueryParameter,
    model::{item::raw, ItemInfo, ItemRarity, SearchItem},
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
        let name = &self.param.item_name;
        if name.is_empty() {
            return Err(InvalidQueryParameter {
                path: "/items".to_owned(),
                message: "`itemName` must be specified.".to_owned(),
            }
            .into());
        }
        let resp = self
            .client
            .get_with_query(
                &format!("/items?itemName={name}", name = encode(name)),
                Some(&self.param),
            )
            .await?;

        Ok(unwrap_rows!(resp, raw::SearchItem => _))
    }

    pub async fn info(&self) -> Result<ItemInfo> {
        let resp = self
            .client
            .get(&format!("/items/{id}", id = self.param.item_id))
            .await?;

        Ok(resp.json().await.unwrap())
    }

    pub async fn multi_info(&self) -> Result<Vec<ItemInfo>> {
        let id = &self.param.item_id;
        if id.is_empty() {
            return Err(InvalidQueryParameter {
                path: "/multi/items".to_owned(),
                message: "`itemIds` must be specified. (use `id_iter()`)".to_owned(),
            }
            .into());
        }
        let resp = self
            .client
            .get(&format!("/multi/items?itemIds={id}"))
            .await?;

        Ok(unwrap_rows!(resp, ItemInfo))
    }

    pub async fn image(&self) -> Result<Bytes> {
        self.client.image()._item(&self.param.item_id).await
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
        self.param
            .query
            .get_or_insert_with(Default::default)
            .max_level = Some(max_level);
        self
    }
    pub fn min_level(&mut self, min_level: u8) -> &mut Self {
        self.param
            .query
            .get_or_insert_with(Default::default)
            .min_level = Some(min_level);
        self
    }
    pub fn rarity(&mut self, rarity: ItemRarity) -> &mut Self {
        self.param.query.get_or_insert_with(Default::default).rarity = Some(rarity);
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
    pub query: Option<Query>,
}

#[derive(Default, Clone)]
pub struct Query {
    pub min_level: Option<u8>,
    pub max_level: Option<u8>,
    pub rarity: Option<ItemRarity>,
}

nested_query!(Query; min_level, max_level, rarity);
