use crate::model::{Item, ItemExt, ItemInfo, ItemWithRarity};

use self::private::Sealed;

pub trait AsItem: Sealed {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}

macro_rules! impl_id {
    ($($ty:ty)*) => {
        $(
            impl Sealed for $ty {}
            impl AsItem for $ty {
                fn id(&self) -> &str {
                    &self.id
                }
                fn name(&self) -> &str {
                    &self.name
                }
            }
        )*
    };
}

impl_id![Item ItemWithRarity ItemInfo ItemExt];

mod private {
    pub trait Sealed {}
}
