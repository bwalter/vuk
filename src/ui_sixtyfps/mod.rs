/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */

use sixtyfps::{Model, ModelHandle, Timer, VecModel};

sixtyfps::sixtyfps! {
    import { MainWindow } from "src/ui_sixtyfps/vuk.60";
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let main_window = MainWindow::new();

    main_window.run();
    Ok(())
}
