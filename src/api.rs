use serde::Serialize;

/// `{ "rows": [ ... ] }` to `[ ... ]`
macro_rules! unwrap_rows {
    ($resp:ident, $ty:ty) => {{
        #[derive(serde::Deserialize)]
        struct __Rows {
            rows: Vec<$ty>,
        }

        $resp.json::<__Rows>().await.unwrap().rows
    }};
}

/// impl Serialize for nested query
/// fields are renamed to camelCase
///
/// `field_a:value_a,field_b:value_b,...`
macro_rules! nested_query {
    ($target:ty; $($field:ident),* $(,)?) => {
        impl serde::Serialize for $target {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use convert_case::{Case, Casing};
                let mut buf = vec![];
                $(
                    if let Some(v) = &self.$field {
                        buf.push(format!(
                            "{}:{}",
                            stringify!($field).to_case(Case::Camel),
                            v,
                        ))
                    }
                )*
                serializer.serialize_str(&buf.join(","))
            }
        }
    };
}

#[derive(Default, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WordType {
    #[default]
    Match,
    Front,
    Full,
}

/// API Endpoints 2~14
///
/// ## Unimplemented
/// - 04. timeline
/// - 05. status
/// - 11. skill style
pub mod character;

/// API Endpoints 15~17
///
/// ## Unimplemented
/// - 16. lookup
pub mod auction;

/// API Endpoints 23~31
///
/// ## Unimplemented
/// - 25. shop
/// - 27. item hashtags
/// - 28 ~ 31. set item
pub mod item;

/// Image API
pub mod image;
