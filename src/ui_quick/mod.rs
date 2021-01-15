extern crate qmetaobject;

use std::path::PathBuf;

use cstr::cstr;
use qmetaobject::*;

use crate::{
    create_ui_controller,
    ui_controller::{UiController, UiListener},
    ui_state::{UiItem, UiSelection},
    TEST_AIDL,
};

qrc!(vuk_resource,
    "vuk" {
        "src/ui_quick/CustomToolButton.qml",
        "src/ui_quick/Connector.qml",
        "src/ui_quick/Diagram.qml",
        "src/ui_quick/DiagramItem.qml",
        "src/ui_quick/Documentation.qml",
        "src/ui_quick/Navigation.qml",
        "src/ui_quick/ItemSymbol.qml",
        "src/ui_quick/OpenFileDialog.qml",
        "src/ui_quick/Selection.qml",
        "src/ui_quick/Style.qml",
        "src/ui_quick/main.qml",
        "src/ui_quick/fonts/segoeui.ttf",
        "src/ui_quick/fonts/segoeuisl.ttf",
        "src/ui_quick/fonts/SegMDL2.ttf",
    },
);

#[derive(QObject, Default)]
struct QuickVuk {
    base: qt_base_class!(trait QObject),

    // Properties
    selection: qt_property!(String; NOTIFY selection_changed),

    // Signals
    selection_changed: qt_signal!(),
    error: qt_signal!(error: String),

    // Slots
    init: qt_method!(
        fn init(&mut self) {
            let listener = Listener {};
            match create_ui_controller(TEST_AIDL) {
                Ok(mut controller) => {
                    controller.add_listener(Box::new(listener));

                    self.selection = serde_json::to_string(&controller.selection).unwrap();
                    self.controller = Some(controller);

                    self.selection_changed();
                }
                Err(e) => {
                    self.error(e.to_string());
                }
            }
        }
    ),

    open: qt_method!(
        fn open(&mut self, url: QString) {
            let path = convert_file_url(&url.to_string());

            println!("Opening {}...", path.to_str().unwrap());

            let listener = Listener {};
            match UiController::open(&path) {
                Ok(mut controller) => {
                    controller.add_listener(Box::new(listener));

                    self.selection = serde_json::to_string(&controller.selection).unwrap();
                    self.controller = Some(controller);

                    self.selection_changed();
                }
                Err(e) => {
                    self.error(e.to_string());
                }
            }
        }
    ),

    get_root_node: qt_method!(
        fn get_root_node(&self, key: QString) -> String {
            if let Some(controller) = &self.controller {
                serde_json::to_string(&controller.create_node(&key.into())).unwrap()
            } else {
                "null".to_string()
            }
        }
    ),

    get_dependency_edges: qt_method!(
        fn get_dependency_edges(&self, key: QString) -> String {
            if let Some(controller) = &self.controller {
                match controller.get_dependencies(&key.into()) {
                    Ok(edges) => serde_json::to_string(&edges).unwrap(),
                    Err(e) => {
                        self.error(e.to_string());
                        "null".to_string()
                    }
                }
            } else {
                "null".to_string()
            }
        }
    ),

    get_dependent_edges: qt_method!(
        fn get_dependent_edges(&self, key: QString) -> String {
            if let Some(controller) = &self.controller {
                match controller.get_references(&key.into()) {
                    Ok(edges) => serde_json::to_string(&edges).unwrap(),
                    Err(e) => {
                        self.error(e.to_string());
                        "null".to_string()
                    }
                }
            } else {
                "null".to_string()
            }
        }
    ),

    // Internal
    controller: Option<UiController>,
}

struct Listener;

impl UiListener for Listener {
    fn on_selection_changed(&self, selection: UiSelection) {}
    fn on_root_changed(&self, root: UiItem) {}
    fn on_item_expanded(&self, item: UiItem) {}
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    vuk_resource();
    init_platform();
    qml_register_type::<QuickVuk>(cstr!("Vuk"), 1, 0, cstr!("Vuk"));

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/vuk/src/ui_quick/main.qml".into());
    engine.exec();

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn init_platform() {
    QQuickStyle::set_style("Fusion");
}

#[cfg(target_os = "windows")]
fn init_platform() {
    //QQuickStyle::set_style("Universal");
}

#[cfg(not(target_os = "windows"))]
fn convert_file_url<'a>(file_url: &str) -> PathBuf {
    file_url.strip_prefix("file://").unwrap().into()
}

#[cfg(target_os = "windows")]
fn convert_file_url<'a>(file_url: &'a str) -> PathBuf {
    use path_slash::PathBufExt;

    let stripped_url = file_url.strip_prefix("file:///").unwrap();
    PathBuf::from_slash(stripped_url)
}
