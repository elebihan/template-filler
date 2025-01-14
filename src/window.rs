//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use crate::application::TemplateFiller;
use crate::document::Document;
use crate::variable::Variable;
use crate::widgets::{VariableNameCell, VariableValueCell, VariablesView};
use glib::clone;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::{cell::RefCell, collections::HashMap, path::Path};
use tracing::{debug, error};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub save_button: gtk::TemplateChild<gtk::Button>,
        pub(crate) document: RefCell<Option<Document>>,
        #[template_child]
        pub(crate) variables_view: gtk::TemplateChild<VariablesView>,
        pub(crate) variables: RefCell<Option<gio::ListStore>>,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                save_button: gtk::TemplateChild::default(),
                document: RefCell::new(None),
                variables_view: gtk::TemplateChild::default(),
                variables: RefCell::new(None),
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
            klass.install_action("win.close-document", None, move |win, _, _| {
                debug!("win.close-document");
                win.close_document()
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_variables();
            self.obj().setup_factories();
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
                        win.open_document(file)
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
            #[weak(rename_to = win)]
            self,
            move |dialog, response| {
                dialog.close();
                if response == gtk::ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            debug!("Saving as {}", path.display());
                            win.save_document(&path);
                        }
                    }
                }
            }
        ));
        dialog.show();
    }

    pub(crate) fn open_document(&self, file: gio::File) {
        match file
            .path()
            .ok_or_else(|| "Invalid file".to_string())
            .and_then(|p| Document::open(p).map_err(|e| e.to_string()))
        {
            Ok(document) => {
                self.load_variables(&document);
                let file_name = document.path().file_name().and_then(|n| n.to_str());
                self.set_title(file_name);
                *self.imp().document.borrow_mut() = Some(document);
                self.imp().save_button.set_visible(true);
                self.action_set_enabled("win.save-document", true)
            }
            Err(error) => error!("open_document: {}", error),
        }
    }

    fn close_document(&self) {
        if self.imp().document.borrow().is_some() {
            self.clear_variables();
            self.set_title(Some("template-filler"));
            *self.imp().document.borrow_mut() = None;
            self.imp().save_button.set_visible(false);
            self.action_set_enabled("win.save-document", false)
        }
    }

    fn load_variables(&self, document: &Document) {
        let variables = self.imp().variables.borrow();
        if let Some(list_store) = variables.as_ref() {
            list_store.remove_all();
            for variable in document.variables() {
                let variable = Variable::new(variable, "");
                list_store.append(&variable);
            }
        }
    }

    fn clear_variables(&self) {
        if let Some(list_store) = self.imp().variables.borrow().as_ref() {
            list_store.remove_all();
        }
    }

    fn save_document(&self, path: &Path) {
        if let Some(document) = self.imp().document.borrow().as_ref() {
            if let Some(list_store) = self.imp().variables.borrow().as_ref() {
                let data: HashMap<String, String> = list_store
                    .iter()
                    .filter_map(|item| {
                        item.ok().and_then(|item: glib::Object| {
                            item.downcast_ref::<Variable>()
                                .map(|variable| (variable.name(), variable.value()))
                        })
                    })
                    .collect();
                if let Err(_err) = document.render_to_file(path, &data) {
                    todo!()
                }
            }
        }
    }

    fn setup_variables(&self) {
        let model = gio::ListStore::new::<Variable>();
        let sorter = self
            .imp()
            .variables_view
            .sorter()
            .expect("VariablesView must have a Sorter");
        let sorted_model = gtk::SortListModel::new(Some(model.clone()), Some(sorter));
        self.imp().variables.replace(Some(model));
        let selection_model = gtk::NoSelection::new(Some(sorted_model));
        self.imp().variables_view.set_model(Some(&selection_model));
    }

    fn setup_factories(&self) {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let variable_cell = VariableNameCell::new();
            list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("VariableNameCell must be a ListItem")
                .set_child(Some(&variable_cell));
        });
        factory.connect_bind(move |_, list_item| {
            let variable = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be a ListItem")
                .item()
                .and_downcast::<Variable>()
                .expect("The item must be a Variable");
            let cell = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be a ListItem")
                .child()
                .and_downcast::<VariableNameCell>()
                .expect("The child must be a VariableNameCell");
            cell.bind(&variable);
        });
        factory.connect_unbind(move |_, list_item| {
            let cell = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<VariableNameCell>()
                .expect("The child must be a VariableNameCell");
            cell.unbind();
        });
        self.imp()
            .variables_view
            .set_name_column_factory(Some(&factory));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let variable_cell = VariableValueCell::new();
            list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("VariableValueCell must be a ListItem")
                .set_child(Some(&variable_cell));
        });
        factory.connect_bind(move |_, list_item| {
            let variable = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be a ListItem")
                .item()
                .and_downcast::<Variable>()
                .expect("The item must be a Variable");
            let cell = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be a ListItem")
                .child()
                .and_downcast::<VariableValueCell>()
                .expect("The child must be a VariableValueCell");
            cell.bind(&variable);
        });
        factory.connect_unbind(move |_, list_item| {
            let cell = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<VariableValueCell>()
                .expect("The child must be a VariableValueCell");
            cell.unbind();
        });
        self.imp()
            .variables_view
            .set_value_column_factory(Some(&factory));
    }
}
