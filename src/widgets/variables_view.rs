//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//
use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/variables_view.ui")]
    #[derive(Default)]
    pub struct VariablesView {
        #[template_child]
        pub column_view: gtk::TemplateChild<gtk::ColumnView>,
        #[template_child]
        pub column_name: gtk::TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub column_value: gtk::TemplateChild<gtk::ColumnViewColumn>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VariablesView {
        const NAME: &'static str = "VariablesView";
        type Type = super::VariablesView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VariablesView {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for VariablesView {}
    impl BoxImpl for VariablesView {}
}

glib::wrapper! {
    pub struct VariablesView(ObjectSubclass<imp::VariablesView>)
        @extends gtk::Widget, gtk::Box;
}

impl VariablesView {
    pub fn set_model(&self, model: Option<&impl glib::object::IsA<gtk::SelectionModel>>) {
        self.imp().column_view.set_model(model)
    }

    pub fn set_name_column_factory(
        &self,
        factory: Option<&impl glib::object::IsA<gtk::ListItemFactory>>,
    ) {
        self.imp().column_name.set_factory(factory)
    }

    pub fn set_value_column_factory(
        &self,
        factory: Option<&impl glib::object::IsA<gtk::ListItemFactory>>,
    ) {
        self.imp().column_value.set_factory(factory)
    }
}
