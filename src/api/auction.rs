use std::fmt::Display;

use serde::Serialize;
use serde_with::SerializeDisplay;
use urlencoding::encode;

use crate::{
    model::{AuctionInfo, ItemRarity, SoldAuctionInfo},
    DfClient, Result,
};

use super::WordType;

#[derive(Clone)]
pub struct AuctionHandler {
    client: DfClient,
    param: AuctionSearchParameter,
}

/// # Constructor
impl AuctionHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        AuctionHandler {
            client,
            param: Default::default(),
        }
    }
}

/// # Send Request
impl AuctionHandler {
    pub async fn search(&self) -> Result<Vec<AuctionInfo>> {
        let url = self.make_url("/auction");

        let resp = self.client.get_with_query(&url, Some(&self.param)).await?;

        Ok(unwrap_rows!(resp, AuctionInfo))
    }

    pub async fn sold(&self) -> Result<Vec<SoldAuctionInfo>> {
        let url = self.make_url("/auction-sold");

        let resp = self
            .client
            .get_with_query(&url, Some(&self.param.to_sold_param()))
            .await?;

        Ok(unwrap_rows!(resp, SoldAuctionInfo))
    }

    fn make_url(&self, path: &str) -> String {
        let mut url = format!("{path}?");

        let name = &self.param.item_name;
        let id = &self.param.item_id;

        if !name.is_empty() {
            url.push_str(&format!("itemName={}", encode(name)));
        } else if !id.is_empty() {
            url.push_str(&format!("itemId={}", id));
        } else {
            panic!("item_id or item_name must be set");
        }
        url
    }
}

/// # Parameter
impl AuctionHandler {
    pub fn param(&mut self, param: AuctionSearchParameter) -> &mut Self {
        self.param = param;
        self
    }

    pub fn limit(&mut self, limit: u16) -> &mut Self {
        self.param.limit = Some(limit);
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.param.sort = sort;
        self
    }

    pub fn sort_by_reinforce(&mut self, sort: SortOrder) -> &mut Self {
        self.param.sort.reinforce = Some(sort);
        self
    }

    pub fn sort_by_unit_price(&mut self, sort: SortOrder) -> &mut Self {
        self.param.sort.unit_price = Some(sort);
        self
    }

    pub fn sort_by_auction_no(&mut self, sort: SortOrder) -> &mut Self {
        self.param.sort.auction_no = Some(sort);
        self
    }

    pub fn id(&mut self, item_id: impl Into<String>) -> &mut Self {
        self.param.item_id = item_id.into();
        self
    }

    pub fn name(&mut self, item_name: impl Into<String>) -> &mut Self {
        self.param.item_name = item_name.into();
        self
    }

    pub fn word_type(&mut self, word_type: WordType) -> &mut Self {
        self.param.word_type = Some(word_type);
        self
    }

    pub fn word_short(&mut self, word_short: bool) -> &mut Self {
        self.param.word_short = Some(word_short);
        self
    }

    pub fn query(&mut self, query: Query) -> &mut Self {
        self.param.query = query;
        self
    }

    pub fn min_level(&mut self, min_level: u8) -> &mut Self {
        self.param.query.min_level = Some(min_level);
        self
    }

    pub fn max_level(&mut self, max_level: u8) -> &mut Self {
        self.param.query.max_level = Some(max_level);
        self
    }

    pub fn level(&mut self, min: u8, max: u8) -> &mut Self {
        self.min_level(min).max_level(max)
    }

    pub fn rarity(&mut self, rarity: ItemRarity) -> &mut Self {
        self.param.query.rarity = Some(rarity);
        self
    }

    pub fn min_reinforce(&mut self, min_reinforce: u8) -> &mut Self {
        self.param.query.min_reinforce = Some(min_reinforce);
        self
    }

    pub fn max_reinforce(&mut self, max_reinforce: u8) -> &mut Self {
        self.param.query.max_reinforce = Some(max_reinforce);
        self
    }

    pub fn reinforce(&mut self, min: u8, max: u8) -> &mut Self {
        self.min_reinforce(min).max_reinforce(max)
    }

    pub fn min_refine(&mut self, min_refine: u8) -> &mut Self {
        self.param.query.min_refine = Some(min_refine);
        self
    }

    pub fn max_refine(&mut self, max_refine: u8) -> &mut Self {
        self.param.query.max_refine = Some(max_refine);
        self
    }

    pub fn refine(&mut self, min: u8, max: u8) -> &mut Self {
        self.min_refine(min).max_refine(max)
    }

    pub fn min_adventure_fame(&mut self, min_adventure_fame: u16) -> &mut Self {
        self.param.query.min_adventure_fame = Some(min_adventure_fame);
        self
    }

    pub fn max_adventure_fame(&mut self, max_adventure_fame: u16) -> &mut Self {
        self.param.query.max_adventure_fame = Some(max_adventure_fame);
        self
    }

    pub fn adventure_fame(&mut self, min: u16, max: u16) -> &mut Self {
        self.min_adventure_fame(min).max_adventure_fame(max)
    }
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionSearchParameter {
    // NOTE:
    // `serde_urlencoded` serialize space to plus sign.
    // neople open api doesn't support plus sign.
    // so we need to add it manually by `urlencoding::encode`.
    #[serde(skip)]
    pub item_name: String,
    #[serde(skip)]
    pub item_id: String,
    pub limit: Option<u16>,
    pub sort: Sort,
    pub word_type: Option<WordType>,
    pub word_short: Option<bool>,
    #[serde(rename = "q")]
    pub query: Query,
}

impl AuctionSearchParameter {
    fn to_sold_param(&self) -> SoldAuctionParam {
        SoldAuctionParam {
            limit: self.limit,
            word_type: self.word_type,
            word_short: self.word_short,
        }
    }
}

#[derive(Default, Clone)]
pub struct Sort {
    pub unit_price: Option<SortOrder>,
    pub reinforce: Option<SortOrder>,
    pub auction_no: Option<SortOrder>,
}

nested_query!(Sort; unit_price, reinforce, auction_no);

#[derive(Clone, SerializeDisplay)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Default, Clone)]
pub struct Query {
    pub min_level: Option<u8>,
    pub max_level: Option<u8>,
    pub rarity: Option<ItemRarity>,
    pub min_reinforce: Option<u8>,
    pub max_reinforce: Option<u8>,
    pub min_refine: Option<u8>,
    pub max_refine: Option<u8>,
    pub min_adventure_fame: Option<u16>,
    pub max_adventure_fame: Option<u16>,
}

nested_query!(
    Query;
    rarity,
    min_level, min_reinforce, min_refine, min_adventure_fame,
    max_level, max_reinforce, max_refine, max_adventure_fame,
);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SoldAuctionParam {
    limit: Option<u16>,
    word_type: Option<WordType>,
    word_short: Option<bool>,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
        .fmt(f)
    }
}
