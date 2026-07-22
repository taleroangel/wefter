use anyhow::Result;
use serde_json::Value;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};
use tera::{Context, Tera};

/// Render a template file (from a [PathBuf]) into a [String] using variables from
pub fn render_file(path: PathBuf, json: Value) -> Result<String> {
    // Read the file
    let template = fs::read_to_string(&path)?;
    let name = path.to_str().unwrap_or("template");

    // Build template
    let mut tera = Tera::default();
    tera.add_raw_template(name, &template)?;

    // Insert parameters into template
    let context = Context::from_serialize(&json)?;
    Ok(tera.render(name, &context)?)
}

/// Render a raw template (from a [String]) into a [String] using variables from
pub fn render_inline(raw: String, json: Value) -> Result<String> {
    // Build template
    let mut tera = Tera::default();
    tera.add_raw_template("inline", &raw)?;

    // Insert parameters into template
    let context = Context::from_serialize(&json)?;
    Ok(tera.render("inline", &context)?)
}

/// Internal helper to embed rendered template text into a file, matching
/// the indentation of the insertion point line.
fn embed_rendered(srcpath: &PathBuf, lookup: &str, rendered: &str) -> Result<()> {
    // Create handle to the src file
    let src = fs::File::open(srcpath)?;
    let read = io::BufReader::new(src);

    // Create handle to the dst (temp) file
    let dstpath = srcpath.with_added_extension(".wefter");
    let dst = fs::File::create(&dstpath)?;
    let mut wrt = io::BufWriter::new(dst);

    // Iterate over lines, for each line in src file, copy it
    // to the dst file. If target insertion point is found, render
    // the template before copying, matching the insertion point's indentation.
    let mut rlines = read.lines();
    while let Some(l) = rlines.next() {
        let l = l?;

        if l.contains(lookup) {
            let indent_len = l.find(|c: char| !c.is_whitespace()).unwrap_or(l.len());
            let indent = &l[..indent_len];

            for line in rendered.lines() {
                if line.is_empty() {
                    writeln!(wrt)?;
                } else {
                    writeln!(wrt, "{}{}", indent, line)?;
                }
            }
        }

        // Copy lines from src to dst
        writeln!(wrt, "{}", l)?;
    }

    // Flush & replace
    wrt.flush()?;
    fs::rename(dstpath, srcpath)?;

    Ok(())
}

/// Embed a file template into an already existing file, into a particular
/// _insertion point_.
///
/// Reads a template with [render_file], using `template` and `tjson` as parameters,
/// looks for all the lines that contain `lookup` (_insertion point_), and
/// inserts the template at the line before the _insertion point_, matching its indentation.
///
/// # Arguments
///
/// * `srcpath` - Path to the file on which the template is going to be embedded.
/// * `lookup` - Insertion Point String. String to look up for within the file,
/// the template is going to be inserted the line before `lookup` is found for
/// every line matching.
/// * `template` - Path to the template file, parameter for [render_file]
/// * `tjson` - Template JSON parameters, parameter for [render_file]
pub fn embed_file(srcpath: PathBuf, lookup: String, template: PathBuf, tjson: Value) -> Result<()> {
    let t = render_file(template, tjson)?;
    embed_rendered(&srcpath, &lookup, &t)
}

/// Embed an inline template into an already existing file, into a particular
/// _insertion point_.
///
/// See [`embed_file`]
pub fn embed_inline(srcpath: PathBuf, lookup: String, template_str: String, tjson: Value) -> Result<()> {
    let t = render_inline(template_str, tjson)?;
    embed_rendered(&srcpath, &lookup, &t)
}
