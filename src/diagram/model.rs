#[derive(Clone, Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Parcelable,
    Interface,
    Enum,
}

impl Item {
    pub fn new<T>(item_type: ItemType, name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            item_type,
            name: name.into(),
        }
    }
}
