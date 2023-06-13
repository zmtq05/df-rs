use convert_case::{Case, Casing};
use serde::Serialize;
use urlencoding::encode;

use crate::{
    model::{AuctionInfo, ItemRarity, SoldAuctionInfo},
    DfClient, DF_BASE_URL,
};

#[derive(Clone)]
pub struct AuctionArtifacts<'df> {
    client: &'df DfClient,

    param: Param,
}

impl<'df> AuctionArtifacts<'df> {
    fn make_url(&self, path: &str) -> String {
        let mut url = format!("{DF_BASE_URL}/{path}?");

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

    pub async fn search(&self) -> crate::Result<Vec<AuctionInfo>> {
        let url = self.make_url("auction");

        let resp = self.client.inner.get(url).query(&self.param).send().await?;
        let resp = crate::map_api_error(resp).await?;

        Ok(unwrap_rows!(resp, AuctionInfo))
    }

    pub async fn sold(&self) -> crate::Result<Vec<SoldAuctionInfo>> {
        let url = self.make_url("auction-sold");

        let resp = self
            .client
            .inner
            .get(url)
            .query(&SoldAuctionParam::from(self.param.clone()))
            .send()
            .await?;

        let resp = crate::map_api_error(resp).await?;

        Ok(unwrap_rows!(resp, SoldAuctionInfo))
    }
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    // NOTE:
    // `serde_urlencoded` serialize space to plus sign.
    // neople open api doesn't support plus sign.
    // so we need to add it manually by `urlencoding::encode`.
    #[serde(skip)]
    item_id: String,

    #[serde(skip)]
    item_name: String,

    limit: Option<u16>,

    #[serde(serialize_with = "Sort::nested_qs")]
    sort: Sort,

    word_type: Option<WordType>,

    word_short: Option<bool>,

    #[serde(rename = "q", serialize_with = "Query::nested_qs")]
    query: Query,
}

#[derive(Default, Clone)]
pub struct Sort {
    pub unit_price: Option<SortOrder>,
    pub reinforce: Option<SortOrder>,
    pub auction_no: Option<SortOrder>,
}

impl Sort {
    fn nested_qs<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = vec![];
        macro_rules! add {
            ($($field:ident),*) => {
                $(
                    match &self.$field {
                        Some(SortOrder::Asc) => buf.push(format!("{}:asc", stringify!($field).to_case(Case::Camel))),
                        Some(SortOrder::Desc) => buf.push(format!("{}:desc", stringify!($field).to_case(Case::Camel))),
                        None => {}
                    }
                )*
            };
        }

        add![unit_price, reinforce, auction_no];

        serializer.serialize_str(buf.join(",").as_str())
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WordType {
    #[default]
    Match,
    Front,
    Full,
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

impl Query {
    fn nested_qs<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = vec![];

        macro_rules! add {
            ($($field:ident),*) => {
                $(
                if let Some(field) = &self.$field {
                    let str = format!(
                        "{k}:{v}",
                        k = stringify!($field).to_case(Case::Camel),
                        v = field,
                    );
                    buf.push(str);
                }
                )*
            };
        }
        add![
            min_level,
            max_level,
            rarity,
            min_reinforce,
            max_refine,
            min_refine,
            min_adventure_fame,
            max_adventure_fame
        ];

        serializer.serialize_str(&buf.join(","))
    }
}

impl<'df> AuctionArtifacts<'df> {
    pub(crate) fn new(client: &'df DfClient) -> Self {
        AuctionArtifacts {
            client,
            param: Default::default(),
        }
    }

    pub fn param(mut self, param: Param) -> Self {
        self.param = param;
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.param.limit = Some(limit);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.param.sort = sort;
        self
    }

    pub fn sort_by_reinforce(mut self, sort: SortOrder) -> Self {
        self.param.sort.reinforce = Some(sort);
        self
    }

    pub fn sort_by_unit_price(mut self, sort: SortOrder) -> Self {
        self.param.sort.unit_price = Some(sort);
        self
    }

    pub fn sort_by_auction_no(mut self, sort: SortOrder) -> Self {
        self.param.sort.auction_no = Some(sort);
        self
    }

    pub fn item_id(mut self, item_id: String) -> Self {
        self.param.item_id = item_id;
        self
    }

    pub fn item_name(mut self, item_name: String) -> Self {
        self.param.item_name = item_name;
        self
    }

    pub fn word_type(mut self, word_type: WordType) -> Self {
        self.param.word_type = Some(word_type);
        self
    }

    pub fn word_short(mut self, word_short: bool) -> Self {
        self.param.word_short = Some(word_short);
        self
    }

    pub fn query(mut self, query: Query) -> Self {
        self.param.query = query;
        self
    }

    pub fn min_level(mut self, min_level: u8) -> Self {
        self.param.query.min_level = Some(min_level);
        self
    }

    pub fn max_level(mut self, max_level: u8) -> Self {
        self.param.query.max_level = Some(max_level);
        self
    }

    pub fn level(self, min: u8, max: u8) -> Self {
        self.min_level(min).max_level(max)
    }

    pub fn rarity(mut self, rarity: ItemRarity) -> Self {
        self.param.query.rarity = Some(rarity);
        self
    }

    pub fn min_reinforce(mut self, min_reinforce: u8) -> Self {
        self.param.query.min_reinforce = Some(min_reinforce);
        self
    }

    pub fn max_reinforce(mut self, max_reinforce: u8) -> Self {
        self.param.query.max_reinforce = Some(max_reinforce);
        self
    }

    pub fn reinforce(self, min: u8, max: u8) -> Self {
        self.min_reinforce(min).max_reinforce(max)
    }

    pub fn min_refine(mut self, min_refine: u8) -> Self {
        self.param.query.min_refine = Some(min_refine);
        self
    }

    pub fn max_refine(mut self, max_refine: u8) -> Self {
        self.param.query.max_refine = Some(max_refine);
        self
    }

    pub fn refine(self, min: u8, max: u8) -> Self {
        self.min_refine(min).max_refine(max)
    }

    pub fn min_adventure_fame(mut self, min_adventure_fame: u16) -> Self {
        self.param.query.min_adventure_fame = Some(min_adventure_fame);
        self
    }

    pub fn max_adventure_fame(mut self, max_adventure_fame: u16) -> Self {
        self.param.query.max_adventure_fame = Some(max_adventure_fame);
        self
    }

    pub fn adventure_fame(self, min: u16, max: u16) -> Self {
        self.min_adventure_fame(min).max_adventure_fame(max)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SoldAuctionParam {
    limit: Option<u16>,
    word_type: Option<WordType>,
    word_short: Option<bool>,
}

impl From<Param> for SoldAuctionParam {
    fn from(value: Param) -> Self {
        let Param {
            limit,
            word_type,
            word_short,
            ..
        } = value;

        SoldAuctionParam {
            limit,
            word_type,
            word_short,
        }
    }
}
