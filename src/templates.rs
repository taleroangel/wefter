use anyhow::Result;
use serde_json::Value;
use std::{fs, io::{self, BufRead, Write}, path::PathBuf};
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

/// Embed a template into an already existing file, into a particular
/// _insertion point_.
///
/// Reads a template with [render], using `template` and `tjson` as parameters,
/// looks for all the lines that contain `lookup` (_insertion point_), and
/// inserts the template at the line before the _insertion point_.
///
/// # Arguments
/// 
/// * `file` - Path to the file on which the template is going to be embedded.
/// * `lookup` - Insertion Point String. String to look up for within the file,
/// the template is going to be inserted the line before `lookup` is found for
/// every line matching.
/// * `template` - Path to the template file, parameter for [render]
/// * `tjson` - Template JSON parameters, parameter for [render]
///
/// # Implementation Notes
///
/// What the code actually does is copy the src file into a temp file line by
/// line, and if the line contains the `lookup` string, then the template is
/// rendered before copying the line with the insertion point
pub fn embed(srcpath: PathBuf, lookup: String, template: PathBuf, tjson: Value) -> Result<()> {

    // Render template
    let t = render(template, tjson)?;

    // Create handle to the src file
    let src = fs::File::open(&srcpath)?;
    let read = io::BufReader::new(src);

    // Create handle to the dst (temp) file
    let dstpath = srcpath.with_added_extension(".loom");
    let dst = fs::File::create(&dstpath)?;
    let mut wrt = io::BufWriter::new(dst);

    // Iterate over lines, for each line in src file, copy it
    // to the dst file. If target insertion point if found, render
    // the template before copying
    let mut rlines = read.lines();
    while let Some(l) = rlines.next() {

        let l = l?;

        // Previous line did have the insertion point
        // write the template at this point
        if l.contains(&lookup) {
            writeln!(wrt, "{}", t)?;
        }

        // Copy lines from src to dst
        writeln!(wrt, "{}", l)?;
    }

    // Flush & replace
    wrt.flush()?;
    fs::rename(dstpath, srcpath)?;

    Ok(())
}
