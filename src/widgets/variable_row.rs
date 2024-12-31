//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use crate::variable::Variable;
use gtk::{
    glib::{self, object::*},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/variable_row.ui")]
    pub struct VariableRow {
        #[template_child]
        pub name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub value_entry: gtk::TemplateChild<gtk::Entry>,
        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VariableRow {
        const NAME: &'static str = "VariableRow";
        type Type = super::VariableRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VariableRow {}
    impl WidgetImpl for VariableRow {}
    impl BoxImpl for VariableRow {}
}

glib::wrapper! {
    pub struct VariableRow(ObjectSubclass<imp::VariableRow>)
        @extends gtk::Widget, gtk::Box;
}

impl VariableRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, variable: &Variable) {
        let mut bindings = self.imp().bindings.borrow_mut();

        let name_label = self.imp().name_label.get();
        let name_label_binding = variable
            .bind_property("name", &name_label, "label")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(name_label_binding);

        let value_entry = self.imp().value_entry.get();
        let value_entry_buffer = value_entry.buffer();
        let value_entry_binding = variable
            .bind_property("value", &value_entry_buffer, "text")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(value_entry_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

impl Default for VariableRow {
    fn default() -> Self {
        Self::new()
    }
}
