use anyhow::Result;
use serde_json::Value;
use std::{fs, path::PathBuf};
use tera::{Context, Tera};

/// Render a template (from a [PathBuf]) into a [String]
pub fn render(path: PathBuf, json: Value) -> Result<String> {
    // Read the file
    let template = fs::read_to_string(&path)?;
    let name = path.to_str().unwrap_or("template");

    // Build template
    let mut tera = Tera::default();
    tera.add_raw_template(name, &template)?;

    // Insert parameters into template
    let context = Context::from_value(json)?;
    Ok(tera.render(name, &context)?)
}
