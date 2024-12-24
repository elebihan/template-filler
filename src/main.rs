//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

mod application;
mod config;
mod window;

use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    gio::resources_register_include!("resources.gresource").expect("Resources should be available");

    glib::set_application_name("TemplateFiller");

    let app = application::TemplateFiller::default();
    app.run()
}
