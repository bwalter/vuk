use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read, path::PathBuf, rc::Rc};
use walkdir::WalkDir;

use crate::parser::{aidl, error::ParseContentError};
use crate::parser::{aidl::ast, error::ParseFileError};
use crate::ui_state::{UiEdge, UiItem, UiNode, UiNodeState, UiSelection};
use crate::{model, model::Model, ui_state::UiSelectionItem};

pub trait UiListener {
    fn on_selection_changed(&self, selection: UiSelection);
    fn on_root_changed(&self, root: UiItem);
    fn on_item_expanded(&self, item: UiItem);
}

pub struct UiController {
    pub model: Model,
    pub selection: UiSelection,
    listeners: Vec<Box<dyn UiListener>>,
}

impl UiController {
    pub fn new(model: Model) -> Self {
        // Create selection items
        let selection = UiSelection {
            items: create_selection_items(&model.items),
            current: -1,
        };

        UiController {
            model,
            selection,
            listeners: Vec::new(),
        }
    }

    pub fn open(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut aidl_file_entries = WalkDir::new(path)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .and_then(|ext| Some(ext == "aidl"))
                    .unwrap_or(false)
            });

        let mut ast_files = Vec::<ast::File>::new();

        aidl_file_entries.try_for_each(|e| {
            let mut file = File::open(e.path())?;

            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            let ast_file = aidl::parse(&buffer).map_err(|content_error: ParseContentError| {
                ParseFileError {
                    path: e.path().into(),
                    content_error,
                }
            })?;

            ast_files.push(ast_file);
            Ok(()) as Result<_, Box<dyn std::error::Error>>
        })?;

        let model = aidl::create_model(ast_files);
        Ok(Self::new(model))
    }

    pub fn add_listener(&mut self, listener: Box<dyn UiListener>) {
        self.listeners.push(listener);
    }

    pub fn create_node(&self, key: &model::Key) -> Option<UiNode> {
        let model_item = self.model.items.get(key);

        if let Some(model_item) = model_item {
            let root_item = UiItem::new_for_model_item(model_item);
            let root_node = UiNode {
                item: root_item,
                state: UiNodeState::Collapsed,
            };

            Some(root_node)
        } else {
            None
        }
    }

    pub fn get_dependencies(
        &self,
        key: &model::Key,
    ) -> Result<Vec<UiEdge>, Box<dyn std::error::Error>> {
        let model_item = self
            .model
            .items
            .get(key)
            .ok_or_else(|| model::Error::NotFound(key.into()))?;

        // Look for sub-nodes
        let model_sub_items = self.model.find_dependencies(model_item);

        let edges = model_sub_items
            .into_iter()
            .map(|(indices, item)| (indices, UiItem::new_for_model_item(&item)))
            .map(|(indices, item)| {
                (
                    indices,
                    UiNode {
                        item,
                        state: UiNodeState::Collapsed,
                    },
                )
            })
            .map(|(indices, node)| UiEdge {
                to: node,
                from_indices: indices,
            })
            .collect();

        Ok(edges)
    }

    pub fn get_references(
        &self,
        key: &model::Key,
    ) -> Result<Vec<UiEdge>, Box<dyn std::error::Error>> {
        let model_item = self
            .model
            .items
            .get(key)
            .ok_or_else(|| model::Error::NotFound(key.into()))?;

        let references = self.model.find_references(model_item);

        let edges = references
            .into_iter()
            .map(|(indices, item)| (indices, UiItem::new_for_model_item(&item)))
            .map(|(indices, item)| {
                (
                    UiNode {
                        item,
                        state: UiNodeState::Collapsed,
                    },
                    indices,
                )
            })
            .map(|(node, indices)| UiEdge {
                to: node,
                from_indices: indices,
            })
            .collect();

        Ok(edges)
    }
}

fn create_selection_items(items: &HashMap<model::Key, Rc<model::Item>>) -> Vec<UiSelectionItem> {
    let mut ret: Vec<UiSelectionItem> = items
        .iter()
        .map(|(_, i)| UiSelectionItem::new_with_model_item(i))
        .collect();

    ret.sort_by(|a, b| {
        let type_ord = a
            .item_type
            .partial_cmp(&b.item_type)
            .unwrap_or(Ordering::Equal);
        match type_ord {
            std::cmp::Ordering::Equal => a.name.partial_cmp(&b.name).unwrap_or(Ordering::Equal),
            _ => type_ord,
        }
    });
    ret
}
