//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use crate::config::APP_ID;
use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};
use tracing::debug;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TemplateFiller;

    #[glib::object_subclass]
    impl ObjectSubclass for TemplateFiller {
        const NAME: &'static str = "TemplateFiller";
        type Type = super::TemplateFiller;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for TemplateFiller {}

    impl ApplicationImpl for TemplateFiller {
        fn activate(&self) {
            debug!("GtkApplication<TemplateFiller>::activate()");
            self.parent_activate();
        }

        fn startup(&self) {
            debug!("GtkApplication<TemplateFiller>::startup()");
            self.parent_startup();
            let app = self.obj();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for TemplateFiller {}
}

glib::wrapper! {
    pub struct TemplateFiller(ObjectSubclass<imp::TemplateFiller>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl TemplateFiller {
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    fn setup_gactions(&self) {
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        self.add_action_entries([action_quit]);
    }
}

impl Default for TemplateFiller {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}
