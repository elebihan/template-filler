//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use crate::config::{APP_ID, VERSION};
use crate::window::Window;
use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};
use tracing::debug;

mod imp {
    use super::*;
    use glib::WeakRef;
    use std::cell::OnceCell;

    #[derive(Debug, Default)]
    pub struct TemplateFiller {
        pub window: OnceCell<WeakRef<Window>>,
    }

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
            self.obj().present_main_window(None);
        }

        fn startup(&self) {
            debug!("GtkApplication<TemplateFiller>::startup()");
            self.parent_startup();
            let app = self.obj();
            gtk::Window::set_default_icon_name(APP_ID);
            app.setup_gactions();
            app.setup_accels();
        }

        fn open(&self, files: &[gio::File], _hint: &str) {
            debug!("GtkApplication<TemplateFiller>::open()");
            let file = files.first().cloned();
            self.obj().present_main_window(file)
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
        self.set_accels_for_action("win.open-document", &["<primary>o"]);
        self.set_accels_for_action("win.save-document", &["<primary>s"]);
        self.set_accels_for_action("win.close-document", &["<primary>w"]);
    }

    fn setup_gactions(&self) {
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let action_about = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about_dialog())
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    fn present_main_window(&self, file: Option<gio::File>) {
        let window = if let Some(window) = self.active_window() {
            window
        } else {
            let window = Window::new(self);
            window.upcast()
        };
        window.present();
        if let Some(file) = file {
            let window = window
                .downcast_ref::<Window>()
                .expect("Widget must be a Window");
            window.open_document(file)
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .program_name("template-filler")
            .logo_icon_name(APP_ID)
            .comments("Render a Handlebars template with user input")
            .version(VERSION)
            .authors(["Eric Le Bihan <eric.le.bihan.dev@free.fr>"])
            .copyright("© 2024 Eric Le Bihan")
            .license_type(gtk::License::MitX11)
            .logo_icon_name(APP_ID)
            .transient_for(
                &self
                    .active_window()
                    .expect("Application should have an active window"),
            )
            .modal(true)
            .build();
        dialog.show();
    }
}

impl Default for TemplateFiller {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build()
    }
}
