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

/// impl Serialize for query string
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

pub mod auction;
pub mod character;
pub mod image;
pub mod item;
