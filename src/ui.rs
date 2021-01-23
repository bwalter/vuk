extern crate qmetaobject;

use cstr::cstr;
use qmetaobject::*;
use std::path::PathBuf;

use crate::{
    create_ui_controller,
    ui_controller::{UiController, UiListener},
    ui_state::{UiItem, UiSelection},
    TEST_AIDL,
};

qrc!(vuk_resource,
    "vuk" {
        "src/ui/qml/Actions.qml",
        "src/ui/qml/ApplicationSettings.qml",
        "src/ui/qml/CustomToolButton.qml",
        "src/ui/qml/Connector.qml",
        "src/ui/qml/Diagram.qml",
        "src/ui/qml/DiagramItem.qml",
        "src/ui/qml/Documentation.qml",
        "src/ui/qml/Navigation.qml",
        "src/ui/qml/ItemSymbol.qml",
        "src/ui/qml/OpenFileDialog.qml",
        "src/ui/qml/Selection.qml",
        "src/ui/qml/main.qml",
        "src/ui/qml/style/qmldir",
        "src/ui/qml/style/Style.qml",
    },
);

// As including the fonts slows down cargo check and build *a lot*, do not
// include them in the debug builds
#[cfg(debug_assertions)]
qrc!(vuk_resource_fonts,
    "vuk_fonts" {}
);

#[cfg(not(debug_assertions))]
qrc!(vuk_resource_fonts,
    "vuk_fonts" {
        "src/ui/fonts/segoeui.ttf",
        "src/ui/fonts/segoeuisl.ttf",
        "src/ui/fonts/SegMDL2.ttf",
    }
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
    vuk_resource_fonts();
    init_platform();

    qml_register_type::<QuickVuk>(cstr!("Vuk"), 1, 0, cstr!("Vuk"));

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/vuk/src/ui/qml/ApplicationSettings.qml".into());
    engine.load_file("qrc:/vuk/src/ui/qml/main.qml".into());
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
