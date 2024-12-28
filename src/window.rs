//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use crate::application::TemplateFiller;
use glib::clone;
use gtk::{glib, prelude::*, subclass::prelude::*};
use tracing::debug;

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub save_button: gtk::TemplateChild<gtk::Button>,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                save_button: gtk::TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("win.open-document", None, move |win, _, _| {
                debug!("win.open-document");
                win.show_open_dialog()
            });
            klass.install_action("win.save-document", None, move |win, _, _| {
                debug!("win.save-document");
                win.show_save_dialog()
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            self.save_button.set_visible(false);
            self.obj().action_set_enabled("win.save-document", false);
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl Window {
    pub fn new(app: &TemplateFiller) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn show_open_dialog(&self) {
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.hbs");
        filter.set_name(Some("Handlebars templates"));
        let dialog = gtk::FileChooserDialog::builder()
            .title("Select Handlebars template")
            .action(gtk::FileChooserAction::Open)
            .filter(&filter)
            .transient_for(self)
            .modal(true)
            .build();
        dialog.add_buttons(&[
            ("_Cancel", gtk::ResponseType::Cancel),
            ("_Open", gtk::ResponseType::Accept),
        ]);
        dialog.connect_response(clone!(
            #[weak(rename_to = win)]
            self,
            move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        debug!("Opening {:?}", file.path());
                        win.imp().save_button.set_visible(true);
                        win.action_set_enabled("win.save-document", true);
                    }
                }
                dialog.close();
            }
        ));
        dialog.show();
    }

    pub fn show_save_dialog(&self) {
        let dialog = gtk::FileChooserDialog::builder()
            .title("Render template as...")
            .action(gtk::FileChooserAction::Save)
            .transient_for(self)
            .modal(true)
            .build();
        dialog.add_buttons(&[
            ("_Cancel", gtk::ResponseType::Cancel),
            ("_Save", gtk::ResponseType::Accept),
        ]);
        dialog.connect_response(clone!(
            #[weak(rename_to = _win)]
            self,
            move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        debug!("Saving as {:?}", file.path());
                    }
                }
                dialog.close();
            }
        ));
        dialog.show();
    }
}
