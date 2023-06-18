use serde::{de::Visitor, Deserialize, Deserializer};

use crate::model::buff::BuffEnhance;

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

/*
before:
{
    "skill": {
        "buff": { // BuffEnhance or null
            "skillInfo": {
                ...
            },
            "creature": {
                ...
            }
        }
    }
}
after:
{
    "buff": {
        "skillInfo": {
            ...
        },
        "creature": {
            ...
        }
    }
}
*/

pub fn flatten_buff_enhance<'de, D>(deserializer: D) -> Result<Option<BuffEnhance>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Outer {
        buff: Option<BuffEnhance>,
    }

    let outer: Outer = Deserialize::deserialize(deserializer)?;
    Ok(outer.buff)
}
