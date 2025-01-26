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
    #[template(resource = "/com/elebihan/TemplateFiller/ui/variable_name_cell.ui")]
    pub struct VariableNameCell {
        #[template_child]
        pub name_inscription: gtk::TemplateChild<gtk::Inscription>,
        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VariableNameCell {
        const NAME: &'static str = "VariableNameCell";
        type Type = super::VariableNameCell;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VariableNameCell {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for VariableNameCell {}
}

glib::wrapper! {
    pub struct VariableNameCell(ObjectSubclass<imp::VariableNameCell>)
        @extends gtk::Widget;
}

impl VariableNameCell {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, variable: &Variable) {
        let mut bindings = self.imp().bindings.borrow_mut();

        let name_inscription = self.imp().name_inscription.get();
        let name_inscription_binding = variable
            .bind_property("name", &name_inscription, "text")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(name_inscription_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

impl Default for VariableNameCell {
    fn default() -> Self {
        Self::new()
    }
}
