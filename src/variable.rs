//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use gtk::{glib, glib::Properties, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

#[derive(Default)]
pub struct VariableData {
    pub name: String,
    pub value: String,
}

mod imp {
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Variable)]
    pub struct Variable {
        #[property(name = "name", get, set, type = String, member = name)]
        #[property(name = "value", get, set, type = String, member = value)]
        pub(crate) data: RefCell<VariableData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Variable {
        const NAME: &'static str = "Variable";
        type Type = super::Variable;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Variable {}
}

glib::wrapper! {
    pub struct Variable(ObjectSubclass<imp::Variable>);
}

impl Variable {
    pub fn new(name: &str, value: &str) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("value", value)
            .build()
    }
}

impl From<VariableData> for Variable {
    fn from(value: VariableData) -> Self {
        Self::new(&value.name, &value.value)
    }
}
