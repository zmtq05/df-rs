use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Slot {
    #[serde(rename = "slotId")]
    pub id: String,
    #[serde(rename = "slotName")]
    pub name: String,
}

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
#[derive(Default, Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Status(pub HashMap<String, StatusValue>);

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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
        #[derive(Debug, Deserialize)]

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

mod character;
pub use character::*;

pub mod item;
pub use item::{
    DungeonObtainInfo, EtcObtainInfo, EtcObtainInfoRow, Explain, GrowInfo, GrowOption,
    GrowOptionDefault, GrowTotal, Item, ItemExt, ItemInfo, ItemRarity, ItemType, ItemTypeDetail,
    ItemWithRarity, ObtainInfo, ReinforceSkill, ReinforceSkillInfo, RuneInfo, SearchItem, Set,
    ShopObtainInfo, TalismanInfo,
};

pub mod auction;
pub use auction::{AuctionItem, AuctionSoldItem};

mod serde_helper;

/// Range: 0~110
type ItemAvailableLevel = u8;
/// Range: 0~20
type Reinforce = u8;
/// Range: 0~8
type Refine = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub enum Amplification {
    None,
    Strength,
    Intelligence,
    Vitality,
    Spirit,
}

impl Display for Amplification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amplification::None => write!(f, ""),
            Amplification::Strength => write!(f, "차원의 힘"),
            Amplification::Intelligence => write!(f, "차원의 지능"),
            Amplification::Vitality => write!(f, "차원의 체력"),
            Amplification::Spirit => write!(f, "차원의 정신력"),
        }
    }
}

impl<'de> Deserialize<'de> for Amplification {
    fn deserialize<D>(deserializer: D) -> Result<Amplification, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Option<&str> = Deserialize::deserialize(deserializer)?;
        match s {
            None => Ok(Amplification::None),
            Some("차원의 힘") => Ok(Amplification::Strength),
            Some("차원의 지능") => Ok(Amplification::Intelligence),
            Some("차원의 체력") => Ok(Amplification::Vitality),
            Some("차원의 정신력") => Ok(Amplification::Spirit),
            Some(s) => Err(serde::de::Error::custom(format!(
                "invalid amplification: {}",
                s
            ))),
        }
    }
}

impl Serialize for Amplification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Amplification::None => serializer.serialize_none(),
            Amplification::Strength => serializer.serialize_str("차원의 힘"),
            Amplification::Intelligence => serializer.serialize_str("차원의 지능"),
            Amplification::Vitality => serializer.serialize_str("차원의 체력"),
            Amplification::Spirit => serializer.serialize_str("차원의 정신력"),
        }
    }
}
