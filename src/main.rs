// Copyright 2020-2021 Benoit Walter

#![recursion_limit = "10240"]

mod model;
mod parser;
mod ui;
mod ui_controller;
mod ui_state;

use crate::parser::aidl;
use crate::ui_controller::UiController;

// TODO:
// x parse interface consts
// - parse annotations
// - parse javadoc
// - parse parcelables
// - parse enums
// - layout
// - filter
// - open file dialog
// - check recursive usage of Rc
// - port to druid

pub fn main() -> jane_eyre::Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Universal");
    std::env::set_var("QT_ENABLE_HIGHDPI_SCALING", "1");
    color_eyre::install()?;
    ui::run()
}

pub fn create_ui_controller(
    aidl: &'static str,
) -> Result<UiController, Box<dyn std::error::Error>> {
    let file = aidl::parse(aidl)?;

    let mut model = aidl::create_model(vec![file]);
    model.resolve_types();

    Ok(UiController::new(model))
}

pub const TEST_AIDL: &'static str = r#"
        package com.concretepage;
        // Krumpli
        /**
         * Prepare a salad.
         *
         * This is probably the greenest food. You need:
         * - a bowl
         * - salat leaves
         * - salt
         * - vinegar
         */
        interface FirstService {
            const int VERSION = 4;
            String getMessage1(String name   );
            String getMessage2(String);
            String getMessage3(String  );
            oneway int getResult(int val1  , Map<String, Vector<int>> val2);
            oneway void useOtherServices(SecondService, ThirdService);
        }

        interface SecondService {
            String getMessage(String name);
            oneway int getResult(int val1, Map<String, Vector<int>> val2);
        }

        interface ThirdService {
            String getMessage(String name);
            oneway int getResult(int val1, Map<String, Vector<int>> val2);
        }"#;
