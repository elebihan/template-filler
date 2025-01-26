//
// This file is part of template-filler
//
// SPDX-FileCopyrightText: © 2024 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::fs;
use std::path::Path;
use std::process::Command;

fn blueprint_compile<P: AsRef<Path>>(
    src_dir: P,
    dst_dir: P,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        if let Some("blp") = src_path.extension().and_then(|ext| ext.to_str()) {
            println!("cargo:rerun-if-changed={}", src_path.to_string_lossy());
            let file_name = src_path.file_name().ok_or("Failed to get file name")?;
            let dst_path = Path::new(dst_dir.as_ref())
                .join(file_name.to_string_lossy().replace(".blp", ".ui"));
            let status = Command::new("blueprint-compiler")
                .arg("compile")
                .arg("--output")
                .arg(dst_path)
                .arg(&src_path)
                .status()?;
            if !status.success() {
                return Err(format!("Failed to compile {}", src_path.display()).into());
            }
        }
    }
    Ok(())
}

fn main() {
    let resources_dir = "data/resources";
    let resources_file = "data/resources/resources.gresource.xml";
    let src_dir = "data/resources/ui";
    let dst_dir = "data/resources/ui";
    println!("cargo:rerun-if-changed={}", resources_file);
    blueprint_compile(src_dir, dst_dir).expect("Blueprint files should be compiled");
    glib_build_tools::compile_resources(&[resources_dir], resources_file, "resources.gresource");
}
