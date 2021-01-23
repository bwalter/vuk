use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::model;

#[derive(Clone, Debug, Default, Serialize)]
pub struct UiSelection {
    pub items: Vec<UiSelectionItem>,
    pub current: i32,
}

impl UiSelection {
    pub fn new_with_model_items(model_items: &HashMap<model::Key, Rc<model::Item>>) -> Self {
        let mut items: Vec<UiSelectionItem> = model_items
                .iter()
                .map(|(_, i)| i)
                .map(UiSelectionItem::new_with_model_item)
                .collect();
        items.sort_by_cached_key(|i| i.pkg_path.clone());
        UiSelection {
            items,
            current: -1,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UiSelectionItem {
    pub item_type: ItemType,
    pub name: String,
    pub pkg_path: String,
    pub key: model::Key,
}

impl UiSelectionItem {
    pub fn new_with_model_item(model_item: &Rc<model::Item>) -> Self {
        UiSelectionItem {
            item_type: ItemType::of_model_item(model_item),
            name: model_item.get_name().to_string(),
            pkg_path: model_item.get_pkg().path.to_string(),
            key: model_item.get_key().clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize)]
pub enum ItemType {
    Interface = 0,
    Struct = 1,
    Enum = 2,
}

impl ItemType {
    fn of_model_item(model_item: &model::Item) -> Self {
        match *model_item {
            model::Item::Interface(_) => ItemType::Interface,
            model::Item::Struct(_) => ItemType::Struct,
            model::Item::Enum(_) => ItemType::Enum,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UiNode {
    pub item: UiItem,
    pub state: UiNodeState,
}

#[derive(Clone, Debug, Serialize)]
pub struct UiItem {
    pub key: model::Key,
    pub item_type: ItemType,
    pub name: String,
    pub docu: String,
    pub members: Vec<UiMember>,
}

impl UiItem {
    pub fn new_for_model_item(model_item: &model::Item) -> Self {
        UiItem {
            key: model_item.get_key().clone(),
            item_type: ItemType::of_model_item(model_item),
            name: model_item.get_name().to_string(),
            docu: model_item.get_docu().to_string(),
            members: match model_item {
                model::Item::Interface(i) => {
                    let const_members = i.consts.iter().map(|c| UiMember::new_from_model_const(c));

                    let method_members =
                        i.methods.iter().map(|m| UiMember::new_from_model_method(m));

                    const_members.chain(method_members).collect()
                }
                model::Item::Struct(s) => s
                    .members
                    .iter()
                    .map(UiMember::new_from_model_member)
                    .collect(),
                model::Item::Enum(e) => e
                    .elements
                    .iter()
                    .map(UiMember::new_from_enum_element)
                    .collect(),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UiMember {
    text: String,
    docu: String,
    index: usize,
}

impl UiMember {
    pub fn new_from_model_const(model_const: &model::Const) -> Self {
        UiMember {
            text: format!(
                "const {}: {} = {}",
                model_const.name,
                model_const.const_type.get_name(),
                model_const.value
            ),
            docu: model_const.docu.clone(),
            index: model_const.index,
        }
    }

    pub fn new_from_model_member(model_member: &model::Member) -> Self {
        UiMember {
            text: format!(
                "{}: {}",
                model_member.arg.name,
                model_member.arg.arg_type.get_name(), // TODO: incl. generic
            ),
            docu: model_member.docu.clone(),
            index: model_member.index,
        }
    }

    pub fn new_from_model_method(model_method: &model::Method) -> Self {
        let arg_string = model_method
            .args
            .iter()
            .map(|arg| {
                //let arg_name = if arg.name.is_empty() { "_" } else { &arg.name };
                //format!("{}: {}", arg_name, arg.arg_type.get_name())
                arg.arg_type.get_name().to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");

        UiMember {
            text: if model_method.return_arg.arg_type.is_void() {
                format!("{} ({})", model_method.name, arg_string)
            } else {
                format!(
                    "{} ({}) -> {}",
                    model_method.name,
                    arg_string,
                    model_method.return_arg.arg_type.get_name()
                )
            },
            docu: model_method.docu.clone(),
            index: model_method.index,
        }
    }

    pub fn new_from_enum_element(element: &model::EnumElement) -> Self {
        UiMember {
            text: element.name.clone(),
            index: element.index,
            docu: element.docu.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum UiNodeState {
    Expanded(Vec<UiEdge>),
    Collapsed,
}

#[derive(Clone, Debug, Serialize)]
pub struct UiEdge {
    pub to: UiNode,
    pub from_indices: HashSet<usize>,
}
