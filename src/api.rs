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

pub mod auction;
pub mod character;
pub mod image;
