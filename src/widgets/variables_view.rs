//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//
use gtk::{glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;
    use crate::variable::Variable;
    use glib::types::StaticType;
    use std::cell::Cell;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/elebihan/TemplateFiller/ui/variables_view.ui")]
    #[properties(wrapper_type = super::VariablesView)]
    pub struct VariablesView {
        #[template_child]
        pub column_view: gtk::TemplateChild<gtk::ColumnView>,
        #[template_child]
        pub column_name: gtk::TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub column_value: gtk::TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub search_bar: gtk::TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_entry: gtk::TemplateChild<gtk::SearchEntry>,
        #[property(get, set)]
        search_mode_enabled: Cell<bool>,
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

    #[glib::derived_properties]
    impl ObjectImpl for VariablesView {
        fn constructed(&self) {
            self.parent_constructed();
            let expression = gtk::PropertyExpression::new(
                Variable::static_type(),
                gtk::Expression::NONE,
                "name",
            );
            let sorter = gtk::StringSorter::builder().expression(expression).build();
            self.column_name.set_sorter(Some(&sorter));
            let expression = gtk::PropertyExpression::new(
                Variable::static_type(),
                gtk::Expression::NONE,
                "value",
            );
            let sorter = gtk::StringSorter::builder().expression(expression).build();
            self.column_value.set_sorter(Some(&sorter));
            self.column_view
                .sort_by_column(Some(&self.column_name), gtk::SortType::Ascending);
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

    pub fn sorter(&self) -> Option<gtk::Sorter> {
        self.imp().column_view.sorter()
    }

    pub fn search_bar(&self) -> gtk::SearchBar {
        self.imp().search_bar.get()
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn toggle_search_bar(&self) {
        let search_bar = self.imp().search_bar.get();
        search_bar.set_search_mode(!search_bar.is_search_mode());
    }
}
