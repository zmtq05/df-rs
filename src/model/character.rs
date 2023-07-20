use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use time::PrimitiveDateTime;

use super::{
    item::{Item, ItemExt, ItemRarity, ItemWithRarity},
    serde_helper, Server, Slot, Status,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

    #[serde_as(deserialize_as = "DefaultOnNull")] // 오래된 캐릭터의 경우 null 일 수 있음
    pub adventure_name: String,
    #[serde(flatten)]
    pub guild: Option<Guild>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Guild {
    #[serde(rename = "guildId")]
    pub id: String,
    #[serde(rename = "guildName")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Job {
    #[serde(rename = "jobId")]
    pub id: String,
    #[serde(rename = "jobName")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct JobGrow {
    #[serde(rename = "jobGrowId")]
    pub id: String,
    #[serde(rename = "jobGrowName")]
    pub name: String,
}

macro_rules! decl_ty_extends_CharacterInfo {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(#[$field_attr:meta])*
            pub $field:ident: $field_type:ty,
        }
    ) => {
        #[serde_with::serde_as]
        $(#[$attr])*
        pub struct $name {
            #[serde(rename = "characterId")]
            pub id: String,
            #[serde(rename = "characterName")]
            pub name: String,

            pub level: u8,

            #[serde(flatten)]
            pub job: crate::model::character::Job,

            #[serde(flatten)]
            pub job_grow: crate::model::character::JobGrow,

            #[serde_as(deserialize_as = "serde_with::DefaultOnNull")] // 오래된 캐릭터의 경우 null 일 수 있음
            pub adventure_name: String,

            #[serde(flatten)]
            pub guild: Option<crate::model::character::Guild>,

            $(#[$field_attr])*
            pub $field: $field_type,
        }
    };
}

decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterEquipments {
        #[serde(rename = "equipment")]
        pub equipments: Vec<Equipment>,
    }
}
decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterAvatars {
        #[serde(rename = "avatar")]
        pub avatars: Vec<Avatar>,
    }
}
decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterCreature {
        pub creature: Option<Creature>,
    }
}
decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterFlag {
        pub flag: Option<Flag>,
    }
}
decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterTalismans {
        #[serde_as(deserialize_as = "DefaultOnNull")]
        pub talismans: Vec<Talisman>,
    }
}
decl_ty_extends_CharacterInfo! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterTimeline {
        pub timeline: Option<Timeline>,
    }
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Enchant {
    pub explain: Option<String>,
    pub status: Option<Status>,
    pub reinforce_skill: Option<Vec<super::item::ReinforceSkill>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct FusionInfo {
    pub options: Vec<FusionOption>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    #[serde(flatten)]
    pub slot: Slot,
    #[serde(flatten)]
    pub item: ItemWithRarity,
    #[serde(deserialize_with = "serde_helper::opt_item", default)]
    pub clone: Option<Item>,
    #[serde(deserialize_with = "serde_helper::opt_item", default)]
    pub random: Option<Item>,
    pub option_ability: Option<String>,
    pub emblems: Vec<Emblem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Emblem {
    pub slot_no: u8,
    pub slot_color: String,
    pub item_name: String,
    pub item_rarity: ItemRarity,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Creature {
    #[serde(flatten)]
    pub item: ItemWithRarity,
    #[serde(deserialize_with = "serde_helper::opt_item", default)]
    pub clone: Option<Item>,
    #[serde(rename = "artifact", default)]
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct CreatureClone {
    pub item_id: Option<String>,
    pub item_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub slot_color: String,
    pub item_name: String,
    pub item_available_level: u8,
    pub item_rarity: ItemRarity,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Gem {
    pub slot_no: u8,
    #[serde(flatten)]
    pub item: ItemWithRarity,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub slot_no: u8,
    #[serde(flatten)]
    pub item: Item,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Talisman {
    pub slot_no: u8,
    pub item: Item,
    pub runes: Vec<Rune>,
}

impl<'de> Deserialize<'de> for Talisman {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_snake_case)]
        #[derive(Deserialize)]
        struct __TalismanInner {
            itemId: String,
            itemName: String,
            slotNo: u8,
        }
        #[derive(Deserialize)]
        struct __Talisman {
            runes: Vec<Rune>,
            talisman: __TalismanInner,
        }

        let __Talisman { runes, talisman } = __Talisman::deserialize(deserializer)?;
        let talisman = Talisman {
            slot_no: talisman.slotNo,
            item: Item {
                id: talisman.itemId,
                name: talisman.itemName,
            },
            runes,
        };

        Ok(talisman)
    }
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct SkillStyleOuter {
    pub style: SkillStyle,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct SkillStyle {
    pub active: Vec<Skill>,
    pub passive: Vec<Skill>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub required_level: u8,
    pub cost_type: SkillCostType,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "UPPERCASE")]
pub enum SkillCostType {
    SP,
    TP,
}

// ------------------------------------

time::serde::format_description!(
    timeline_date_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]"
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct TimelineDate {
    #[serde(with = "timeline_date_format")]
    pub start: PrimitiveDateTime,
    #[serde(with = "timeline_date_format")]
    pub end: PrimitiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct TimelineRow {
    pub code: u16,
    pub name: String,
    pub date: String,
    pub data: serde_json::Map<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Timeline {
    pub date: TimelineDate,
    pub next: Option<String>,
    pub rows: Vec<TimelineRow>,
}

pub mod buff {
    use serde::{Deserialize, Serialize};

    use crate::model::serde_helper;

    decl_ty_extends_CharacterInfo! {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[cfg_attr(feature = "typescript", derive(specta::Type))]
        #[serde(rename_all = "camelCase")]
        pub struct CharacterBuffEnhance {
            #[serde(deserialize_with = "serde_helper::flatten_buff_enhance", rename(deserialize = "skill", serialize = "skillInfo"))]
            pub buff: Option<BuffEnhance>,
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
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

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
    #[serde(rename_all = "camelCase")]
    pub struct SkillInfo {
        #[serde(rename = "skillId")]
        pub id: String,
        pub name: String,
        pub option: SkillOption,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[cfg_attr(feature = "typescript", derive(specta::Type))]
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
