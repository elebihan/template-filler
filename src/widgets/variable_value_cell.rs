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
    subclass::prelude::*,
};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/variable_value_cell.ui")]
    pub struct VariableValueCell {
        #[template_child]
        pub value_label: gtk::TemplateChild<gtk::EditableLabel>,
        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VariableValueCell {
        const NAME: &'static str = "VariableValueCell";
        type Type = super::VariableValueCell;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VariableValueCell {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for VariableValueCell {}
}

glib::wrapper! {
    pub struct VariableValueCell(ObjectSubclass<imp::VariableValueCell>)
        @extends gtk::Widget;
}

impl VariableValueCell {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, variable: &Variable) {
        let mut bindings = self.imp().bindings.borrow_mut();

        let value_label = self.imp().value_label.get();
        let value_label_binding = variable
            .bind_property("value", &value_label, "text")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(value_label_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

impl Default for VariableValueCell {
    fn default() -> Self {
        Self::new()
    }
}
