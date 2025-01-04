//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use handlebars::{
    template::{Parameter, TemplateElement},
    Handlebars, Path as JsonPath, PathSeg,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors reported when handling a document.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid template: {0}")]
    InvalidTemplate(PathBuf),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Render error: {0}")]
    Render(#[from] handlebars::RenderError),
    #[error("Template error: {0}")]
    Template(#[from] handlebars::TemplateError),
}

/// Hold information about a Handlebars template.
#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    variables: Vec<String>,
}

impl Document {
    /// Create a new document for Handlebars template at `path`.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let variables = Self::collect_variables(path)?;
        Ok(Self {
            path: path.into(),
            variables,
        })
    }
    /// Collect the variables in the underlying template as a list.
    fn collect_variables(path: &Path) -> Result<Vec<String>, Error> {
        let name = path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| Error::InvalidTemplate(path.into()))?;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file(name, path)?;
        let template = handlebars
            .get_template(name)
            .expect("Template should have been registered");
        let variables: Vec<String> = template
            .elements
            .iter()
            .filter_map(|e| {
                if let TemplateElement::Expression(h) = e {
                    Some(h)
                } else {
                    None
                }
            })
            .filter_map(|h| {
                if let Parameter::Path(JsonPath::Relative(p)) = &h.name {
                    Some(p)
                } else {
                    None
                }
            })
            .flat_map(|p| &p.0)
            .filter_map(|s| {
                if let PathSeg::Named(n) = s {
                    Some(n)
                } else {
                    None
                }
            })
            .cloned()
            .collect();
        Ok(variables)
    }
    /// Return the path of the document.
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    /// Render the underlying template to a file at `path`, using `data`.
    pub fn render_to_file<P: AsRef<Path>>(
        &self,
        path: P,
        data: &HashMap<String, String>,
    ) -> Result<(), Error> {
        let contents = fs::read_to_string(&self.path)?;
        let output = fs::File::create(path)?;
        let handlebars = Handlebars::new();
        handlebars.render_template_to_write(&contents, data, output)?;
        Ok(())
    }
    /// Return the list of variables in the underlying template.
    pub fn variables(&self) -> impl Iterator<Item = &str> {
        self.variables.iter().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    const TEMPLATE_VALID: &str = r#"""
- firstname: {{firstname}}
  lastname: {{lastname}}
"""#;
    const VARIABLES_VALID: &[&str] = &["firstname", "lastname"];

    #[test]
    fn get_variables_valid() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("template-valid.yml.hbs");
        let mut input = File::create(&path).unwrap();
        writeln!(input, "{}", TEMPLATE_VALID).unwrap();
        let res = Document::open(path);
        assert!(res.is_ok());
        let doc = res.unwrap();
        let variables: Vec<&str> = doc.variables().collect();
        assert_eq!(&variables, VARIABLES_VALID);
    }
}
