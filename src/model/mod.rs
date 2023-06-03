use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Server {
    All,
    Anton,
    Bakal,
    Cain,
    Casillas,
    Diregie,
    Hilder,
    Prey,
    Siroco,
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::All => write!(f, "all"),
            Server::Anton => write!(f, "anton"),
            Server::Bakal => write!(f, "bakal"),
            Server::Cain => write!(f, "cain"),
            Server::Casillas => write!(f, "casillas"),
            Server::Diregie => write!(f, "diregie"),
            Server::Hilder => write!(f, "hilder"),
            Server::Prey => write!(f, "prey"),
            Server::Siroco => write!(f, "siroco"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Slot {
    #[serde(rename = "slotId")]
    pub id: String,
    #[serde(rename = "slotName")]
    pub name: String,
}

mod character {
    use std::{collections::HashMap, fmt::Display};

    use serde::Deserialize;

    use super::{
        item::{Item, ItemExt, ItemRarity, ItemWithRarity},
        serde_helper, Server, Slot,
    };

    #[derive(Clone, Deserialize)]
    pub struct Character {
        #[serde(rename = "characterId")]
        pub id: String,
        #[serde(rename = "characterName")]
        pub name: String,
        #[serde(rename = "serverId")]
        pub server: Server,

        pub level: u8,

        #[serde(flatten)]
        pub job: Job,
        #[serde(flatten)]
        pub job_grow: JobGrow,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterInfo {
        #[serde(rename = "characterId")]
        pub id: String,
        #[serde(rename = "characterName")]
        pub name: String,

        pub level: u8,

        #[serde(flatten)]
        pub job: Job,

        #[serde(flatten)]
        pub job_grow: JobGrow,

        #[serde(default)] // 오래된 캐릭터의 경우 null 일 수 있음
        pub adventure_name: String,

        pub guild: Option<Guild>,
    }

    #[derive(Clone, Deserialize)]
    pub struct Guild {
        #[serde(rename = "guildId")]
        pub id: String,
        #[serde(rename = "guildName")]
        pub name: String,
    }

    #[derive(Clone, Deserialize)]
    pub struct Job {
        #[serde(rename = "jobId")]
        pub id: String,
        #[serde(rename = "jobName")]
        pub name: String,
    }

    #[derive(Clone, Deserialize)]
    pub struct JobGrow {
        #[serde(rename = "jobGrowId")]
        pub id: String,
        #[serde(rename = "jobGrowName")]
        pub name: String,
    }

    // ------------------------------------

    /// ## List of keys - CharacterStatus
    ///
    /// - HP
    /// - MP
    /// - 물리 방어율
    /// - 마법 방어율
    /// - 힘
    /// - 지능
    /// - 체력
    /// - 정신력
    /// - 물리 공격
    /// - 마법 공격
    /// - 물리 크리티컬
    /// - 마법 크리티컬
    /// - 독립 공격
    /// - 공격 속도
    /// - 캐스팅 속도
    /// - 이동 속도
    /// - HP 회복량
    /// - 경직도
    /// - 히트리커버리
    /// - 화속성 강화
    /// - 화속성 저항
    /// - 수속성 강화
    /// - 수속성 저항
    /// - 명속성 강화
    /// - 명속성 저항
    /// - 암속성 강화
    /// - 암속성 저항
    /// - 물리 방어
    /// - 마법 방어
    /// - 피해 증가
    /// - 피해 증가 %
    /// - 버프력
    /// - 버프력 %
    /// - 스킬 공격력 증가
    /// - 쿨타임 감소
    /// - 쿨타임 회복속도 증가
    /// - 쿨타임 감소 실적용
    /// - 데미지 증가
    /// - 크리티컬 데미지 증가
    /// - 추가 데미지 증가
    /// - 모든 공격력 증가
    /// - 물리 공격력 증가
    /// - 마법 공격력 증가
    /// - 독립 공격력 증가
    /// - 힘 증가
    /// - 지능 증가
    /// - 지속피해
    /// - 물리 피해 감소
    /// - 마법 피해 감소
    /// - 출혈 데미지 전환
    /// - 중독 데미지 전환
    /// - 화상 데미지 전환
    /// - 감전 데미지 전환
    /// - 출혈 내성
    /// - 중독 내성
    /// - 감전 내성
    /// - 빙결 내성
    /// - 둔화 내성
    /// - 기절 내성
    /// - 저주 내성
    /// - 암흑 내성
    /// - 석화 내성
    /// - 수면 내성
    /// - 혼란 내성
    /// - 구속 내성
    ///
    /// ## List of key - Enchant
    ///
    /// TODO
    #[derive(Clone)]
    pub struct Status(pub HashMap<String, StatusValue>);

    #[derive(Clone)]
    pub struct StatusValue {
        pub value: f64,
        pub suffix: Option<char>,
    }

    impl Display for StatusValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(suffix) = self.suffix {
                write!(f, "{}{}", self.value, suffix)
            } else {
                write!(f, "{}", self.value)
            }
        }
    }

    impl std::ops::Deref for Status {
        type Target = HashMap<String, StatusValue>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]

            struct StatusInner {
                name: String,
                value: serde_json::Value,
            }
            let map = Vec::<StatusInner>::deserialize(deserializer)?
                .into_iter()
                .map(|inner| {
                    let k = inner.name;
                    let v = match inner.value {
                        serde_json::Value::String(mut v) => match v.pop() {
                            Some(suffix) if suffix == '%' => StatusValue {
                                value: v.parse().unwrap(),
                                suffix: Some('%'),
                            },
                            Some(_) => panic!("value should be ends with '%'"),
                            None => panic!("value should not be empty"),
                        },
                        serde_json::Value::Number(v) => StatusValue {
                            value: v.as_f64().unwrap(),
                            suffix: None,
                        },
                        _ => panic!("value should be string or number"),
                    };
                    (k, v)
                })
                .collect();
            Ok(Self(map))
        }
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Equipment {
        #[serde(flatten)]
        pub slot: Slot,

        #[serde(flatten)]
        pub item: ItemExt,

        pub set_item_id: Option<String>,
        pub set_item_name: Option<String>,

        pub item_grade_name: Option<String>,

        pub enchant: Option<Enchant>,

        pub grow_info: Option<super::item::GrowInfo>,

        pub upgrade_info: Option<Item>,

        pub ispins_info: Option<FusionInfo>,
        pub machine_revolution_info: Option<FusionInfo>,
        pub dimension_cloister_info: Option<FusionInfo>,
        pub bakal_info: Option<FusionInfo>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Enchant {
        pub explain: Option<String>,
        pub status: Option<Status>,
        pub reinforce_skill: Option<Vec<super::item::ReinforceSkill>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct FusionInfo {
        pub options: Vec<FusionOption>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FusionOption {
        #[serde(default)]
        pub damage: u16,
        pub buff: u16,
        pub explain: String,
        pub explain_detail: String,
    }

    // ------------------------------------
    /*
       #[derive(Clone, Deserialize)]
       #[serde(rename_all = "camelCase")]
       pub struct SetItemInfo {
           #[serde(rename = "setItemId")]
           pub id: String,

           #[serde(rename = "setItemName")]
           pub name: String,

           pub slot_info: Vec<ItemWithRarity>,
           pub active_set_no: u8,
       }
    */
    // ------------------------------------

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Avatar {
        #[serde(flatten)]
        pub slot: Slot,
        pub item: ItemWithRarity,
        #[serde(deserialize_with = "serde_helper::opt_item", default)]
        pub clone: Option<Item>,
        #[serde(deserialize_with = "serde_helper::opt_item", default)]
        pub random: Option<Item>,
        pub option_ability: Option<String>,
        pub emblems: Vec<Emblem>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Emblem {
        pub slot_no: u8,
        pub slot_color: String,
        pub item_name: String,
        pub item_rarity: ItemRarity,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Creature {
        #[serde(flatten)]
        pub item: ItemWithRarity,
        pub clone: CreatureClone,
        #[serde(rename = "artifact")]
        pub artifacts: Vec<Artifact>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreatureClone {
        pub item_id: Option<String>,
        pub item_name: Option<String>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artifact {
        pub slot_color: String,
        pub item_name: String,
        pub item_available_level: u8,
        pub item_rarity: ItemRarity,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Flag {
        #[serde(flatten)]
        pub item: ItemWithRarity,
        pub reinforce: u8,
        /// ## keys
        ///
        /// - 피해 증가
        /// - 버프력
        /// - 모험가 명성
        pub reinforce_status: Status,
        pub gems: Vec<Gem>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Gem {
        pub slot_no: u8,
        #[serde(flatten)]
        pub item: ItemWithRarity,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    pub struct TalismanWithRunes {
        pub talisman: Talisman,
        pub runes: Vec<Rune>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Talisman {
        pub slot_no: u8,
        #[serde(flatten)]
        pub item: Item,
        pub rune_types: Vec<String>,
    }

    #[derive(Clone, Deserialize)]
    pub struct Rune {
        pub slot_no: u8,
        #[serde(flatten)]
        pub item: Item,
    }

    // ------------------------------------

    /*
    {
        "skill": {
            "style": {
                "active": [<Skill>],
                "passive": [<Skill>]
            }
        }
    }

    NOTE: just return SkillStyle
    */
    #[derive(Clone, Deserialize)]
    pub struct SkillStyleOuter {
        pub style: SkillStyle,
    }

    #[derive(Clone, Deserialize)]
    pub struct SkillStyle {
        pub active: Vec<Skill>,
        pub passive: Vec<Skill>,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Skill {
        pub id: String,
        pub name: String,
        pub level: u8,
        pub required_level: u8,
        pub cost_type: SkillCostType,
    }

    #[derive(Clone, Copy, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum SkillCostType {
        SP,
        TP,
    }

    mod buff {
        use serde::Deserialize;

        use crate::model::serde_helper;

        #[derive(Clone, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct BuffEnhance {
            #[serde(rename = "skillInfo")]
            pub skill: Option<SkillInfo>,

            #[serde(rename = "equipment")]
            pub equipments: Option<Vec<super::Equipment>>,

            #[serde(rename = "avatar")]
            pub avatars: Option<Vec<super::Avatar>>,

            #[serde(deserialize_with = "serde_helper::creature_vec_pop", default)]
            pub creature: Option<super::Creature>,
        }

        #[derive(Debug, Clone, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SkillInfo {
            #[serde(rename = "skillId")]
            pub id: String,
            pub name: String,
            pub option: SkillOption,
        }

        #[derive(Debug, Clone, Deserialize)]
        pub struct SkillOption {
            // skill level
            pub level: u8,

            // format string
            //
            // ex) "지속 시간 : {value1}초\n공격력 비율 : 타격량의 {value2}%"
            pub desc: String,

            // format string arguments
            //
            // ex) [ "-", "68.2" ]
            pub values: Vec<String>,
        }
    }
}
pub use character::*;

mod item {
    use std::{fmt::Display, str::FromStr};

    use serde::Deserialize;
    use serde_with::DeserializeFromStr;

    use super::serde_helper;

    #[derive(Clone, Deserialize)]
    pub struct Item {
        #[serde(rename = "itemId")]
        pub id: String,
        #[serde(rename = "itemName")]
        pub name: String,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ItemWithRarity {
        #[serde(rename = "itemId")]
        pub id: String,
        #[serde(rename = "itemName")]
        pub name: String,
        #[serde(rename = "itemRarity")]
        pub rarity: ItemRarity,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ItemExt {
        #[serde(rename = "itemId")]
        pub id: String,
        #[serde(rename = "itemName")]
        pub name: String,
        #[serde(rename = "itemRarity")]
        pub rarity: ItemRarity,
        #[serde(flatten)]
        pub r#type: ItemType,
        pub refine: u8,
        pub reinforce: u8,
        pub amplification_name: Option<String>,
        #[serde(rename = "itemAvailableLevel")]
        pub available_level: u8,
    }

    #[derive(Clone, Copy, PartialEq, Eq, DeserializeFromStr)]
    pub enum ItemRarity {
        Common,
        Uncommon,
        Rare,
        Unique,
        Chronicle,
        Legendary,
        Epic,
        Mythic,
    }

    impl FromStr for ItemRarity {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "커먼" => Ok(ItemRarity::Common),
                "언커먼" => Ok(ItemRarity::Uncommon),
                "레어" => Ok(ItemRarity::Rare),
                "유니크" => Ok(ItemRarity::Unique),
                "크로니클" => Ok(ItemRarity::Chronicle),
                "레전더리" => Ok(ItemRarity::Legendary),
                "에픽" => Ok(ItemRarity::Epic),
                "신화" => Ok(ItemRarity::Mythic),
                _ => Err(format!("unknown rarity: {}", s)),
            }
        }
    }

    impl Display for ItemRarity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = match self {
                ItemRarity::Common => "커먼",
                ItemRarity::Uncommon => "언커먼",
                ItemRarity::Rare => "레어",
                ItemRarity::Unique => "유니크",
                ItemRarity::Chronicle => "크로니클",
                ItemRarity::Legendary => "레전더리",
                ItemRarity::Epic => "에픽",
                ItemRarity::Mythic => "신화",
            };

            f.write_str(str)
        }
    }

    #[derive(Clone, Deserialize)]
    pub struct ItemType {
        #[serde(rename = "itemTypeId")]
        pub id: String,
        #[serde(rename = "itemType", default)]
        pub name: String,
        #[serde(flatten)]
        pub detail: ItemTypeDetail,
    }

    #[derive(Clone, Deserialize)]
    pub struct ItemTypeDetail {
        #[serde(rename = "itemTypeDetailId")]
        pub id: String,
        #[serde(rename = "itemTypeDetail", default)]
        pub name: String,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ItemInfo {
        #[serde(rename = "itemId")]
        pub id: String,

        #[serde(rename = "itemName")]
        pub name: String,

        #[serde(rename = "itemRarity")]
        pub rarity: ItemRarity,

        #[serde(flatten)]
        pub r#type: ItemType,

        #[serde(rename = "itemAvailableLevel")]
        pub available_level: u8,

        #[serde(flatten)]
        pub explain: Explain,

        #[serde(rename = "itemFlavorText")]
        pub flavor_text: String,

        pub obtain_info: ObtainInfo,

        #[serde(flatten)]
        pub set: Option<Set>,

        #[serde(rename = "itemStatus")]
        pub status: super::character::Status,

        pub grow_info: Option<GrowInfo>,

        #[serde(rename = "hashtag")]
        pub hashtags: Option<Vec<String>>,

        #[serde(rename = "itemReinforceSkill")]
        pub reinforce_skill: Option<Vec<ReinforceSkill>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct Explain {
        #[serde(rename = "itemExplain")]
        pub value: String,
        #[serde(rename = "itemExplainDetail")]
        pub detail: String,
    }

    // ------------------------------------ ObtainInfo START

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ObtainInfo {
        pub dungeon: Option<Vec<DungeonObtainInfo>>,

        #[serde(deserialize_with = "serde_helper::flatten_shop_obtain_info")]
        pub shop: Option<Vec<ShopObtainInfo>>,

        pub etc: Option<Vec<EtcObtainInfo>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct DungeonObtainInfo {
        // dungeon type
        //
        // 일반 던전, 상급 던전, 레기온, 레이드
        pub r#type: String,

        #[serde(deserialize_with = "serde_helper::flatten_rows", rename = "dungeon")]
        pub dungeons: Vec<String>,
    }

    #[derive(Clone, Deserialize)]
    pub struct ShopObtainInfo {
        // ex) 모험단 상점, NPC 린지 로섬, ...
        pub name: String,
        // ex) ["105레벨 에픽 장비 선택 상자"]
        pub details: Vec<String>,
    }

    #[derive(Clone, Deserialize)]
    pub struct EtcObtainInfo {
        pub name: String,
        pub rows: Vec<EtcObtainInfoRow>,
    }

    #[derive(Clone, Deserialize)]
    pub struct EtcObtainInfoRow {
        pub name: String,
        pub details: Option<Vec<String>>,
    }

    // ------------------------------------ ObtainInfo END

    #[derive(Clone, Deserialize)]
    pub struct Set {
        #[serde(rename = "setItemId")]
        pub id: String,
        #[serde(rename = "setItemName")]
        pub name: String,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    pub struct ReinforceSkill {
        #[serde(flatten)]
        pub job: super::character::Job,
        pub skiils: Vec<ReinforceSkillInfo>,
    }

    #[derive(Clone, Deserialize)]
    pub struct ReinforceSkillInfo {
        #[serde(rename = "skillId")]
        pub id: String,
        pub name: String,
        #[serde(deserialize_with = "serde_helper::str_as_u8")]
        pub value: u8,
    }

    // ------------------------------------

    #[derive(Clone, Deserialize)]
    pub struct GrowInfo {
        pub transfer: Option<bool>,
        pub total: GrowTotal,
        pub options: Vec<GrowOption>,
    }

    #[derive(Clone, Deserialize)]
    pub struct GrowTotal {
        pub damage: i32,
        pub buff: i32,
        pub level: i32,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GrowOption {
        pub level: u8,
        pub exp_rate: f32,
        pub damage: i32,
        pub buff: i32,
        pub explain: String,
        pub explain_detail: String,
        pub default: Option<GrowOptionDefault>,
        pub transfer: Option<bool>,
    }

    #[derive(Clone, Deserialize)]
    pub struct GrowOptionDefault {
        pub damage: i32,
        pub buff: i32,
    }
}
pub use item::*;

mod auction {
    use serde::Deserialize;

    use super::item::ItemExt;

    // TODO: use DateTime
    // format: "yyyy-MM-dd hh:mm:ss"
    type Date = String;

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuctionInfo {
        #[serde(rename = "auctionNo")]
        pub no: u32,
        pub reg_date: Date,
        pub expire_date: Date,
        #[serde(flatten)]
        pub item: ItemExt,
        pub adventure_fame: u16,
        pub count: u32,
        pub current_price: u32,
        pub unit_price: u32,
        pub average_price: u32,
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SoldAuctionInfo {
        pub sold_date: Date,
        #[serde(flatten)]
        pub item: ItemExt,
        pub count: u32,
        pub price: u32,
        pub unit_price: u32,
    }
}
pub use auction::*;

mod serde_helper {
    use serde::{de::Visitor, Deserialize, Deserializer};

    use super::{
        character::Creature,
        item::{Item, ShopObtainInfo},
    };

    pub fn str_as_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrAsU8Visitor;

        impl<'de> Visitor<'de> for StrAsU8Visitor {
            type Value = u8;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer or a string that can be converted to an integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.parse().unwrap())
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.try_into().unwrap())
            }
        }

        deserializer.deserialize_any(StrAsU8Visitor)
    }

    pub fn opt_item<'de, D>(deserializer: D) -> Result<Option<Item>, D::Error>
    where
        D: Deserializer<'de>,
    {
        /*
        "clone": {
            "itemId": null,
            "itemName": null
        }

        unwrap to Option<Item>
         */
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct InnerNullItem {
            item_id: Option<String>,
            item_name: Option<String>,
        }

        let i = InnerNullItem::deserialize(deserializer)?;

        let Some(id) = i.item_id else {
            return Ok(None);
        };
        let Some(name) = i.item_name else {
            return Ok(None);
        };

        Ok(Some(Item { id, name }))
    }

    pub fn creature_vec_pop<'de, D>(deserializer: D) -> Result<Option<Creature>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let creatures: Option<Vec<Creature>> = Deserialize::deserialize(deserializer)?;
        match creatures {
            Some(mut arr) => match arr.pop() {
                Some(creature) => Ok(Some(creature)),
                None => panic!("buff creature should be at least one"),
            },
            None => Ok(None),
        }
    }

    pub fn flatten_rows<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Row {
            name: String,
        }

        // [ { "name": "name1" }, { "name": "name2" } ... ]]
        // ->
        // [ "name1", "name2" ... ]

        let rows: Vec<Row> = Deserialize::deserialize(deserializer)?;

        Ok(rows.into_iter().map(|row| row.name).collect())
    }

    pub fn flatten_shop_obtain_info<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<ShopObtainInfo>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Outer {
            rows: Vec<ShopObtainInfo>,
        }

        /*
        origin:
        "shop": [{ "rows": [
            {
                "name": "A",
                "details": ["B", "C"]
            }
        ]}]

        after:
        "shop": [
            {
                "name": "A",
                "details": ["B", "C"]
            }
        ]
        */

        let arr: Vec<Outer> = Deserialize::deserialize(deserializer)?;
        Ok(Some(arr.into_iter().flat_map(|row| row.rows).collect()))
    }
}
