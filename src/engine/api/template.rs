use super::*;
use crate::{
    ResourceDir,
    fs::hist::{HistoryAction, HistoryRef},
    templates,
};
use std::{fs, path::PathBuf};

/// String for embedding into files
const LUA_WEFTER_TEMPLATE_EMBEDDING_POINT: &str = "@wefter.embed";

// Create a table for the 'template' submodule
pub fn module(l: &Lua, profile: ResourceDir, history: HistoryRef) -> Result<WefterModuleTable<'_>> {
    Ok(vec![
        ("get", {
            let profile = profile.clone();
            l.create_function(move |lua, (template, params): (PathBuf, Table)| {
                let template = profile.build_template_path(template)?;
                let params = serialize_table(lua, params)?;

                log::debug!("[wefter.template.get] template {:?}", template);
                let rendered = templates::render_file(template, params)?;
                Ok(rendered)
            })?
        }),
        ("create", {
            let profile = profile.clone();
            let history = history.clone();
            l.create_function(
                move |lua, (dst, template, params): (PathBuf, PathBuf, Table)| {
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;
                    log::debug!(
                        "[wefter.template.create] Creating file {:?} with template file {:?}",
                        dst,
                        template
                    );

                    // Get file contents
                    let rendered = templates::render_file(template, params)?;

                    // Create parent directory
                    if let Some(parent) = dst.as_path().parent()
                        && !parent.exists()
                    {
                        fs::create_dir_all(parent)?;
                    }

                    // Create file
                    fs::write(dst.clone(), rendered)?;
                    history.borrow_mut().push(HistoryAction::CreateFile(dst));

                    Ok(())
                },
            )?
        }),
        ("create_inline", {
            let history = history.clone();
            l.create_function(
                move |lua, (dst, template_str, params): (PathBuf, String, Table)| {
                    let params = serialize_table(lua, params)?;
                    log::debug!(
                        "[wefter.template.create] Creating file {:?} with inline template",
                        dst,
                    );

                    // Get file contents
                    let rendered = templates::render_inline(template_str, params)?;

                    // Create parent directory
                    if let Some(parent) = dst.as_path().parent()
                        && !parent.exists()
                    {
                        fs::create_dir_all(parent)?;
                    }

                    // Create file
                    fs::write(dst.clone(), rendered)?;
                    history.borrow_mut().push(HistoryAction::CreateFile(dst));

                    Ok(())
                },
            )?
        }),
        ("embed", {
            let profile = profile.clone();
            let history = history.clone();
            l.create_function(
                move |lua,
                      (dst, ipoint, template, params): (
                    PathBuf,
                    Option<String>,
                    PathBuf,
                    Table,
                )| {
                    // Insertion point builder
                    let lookup = match ipoint {
                        Some(e) => format!("{}:{}", LUA_WEFTER_TEMPLATE_EMBEDDING_POINT, e),
                        None => format!("{}", LUA_WEFTER_TEMPLATE_EMBEDDING_POINT),
                    };
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;

                    log::debug!(
                        "[wefter.template.embed] template file {:?} into {:?} at {:?}",
                        template,
                        dst,
                        lookup
                    );

                    // Add contents to file
                    templates::embed_file(dst.clone(), lookup.clone(), template, params)?;
                    history
                        .borrow_mut()
                        .push(HistoryAction::ModifyFile(dst, lookup));

                    Ok(())
                },
            )?
        }),
        ("embed_inline", {
            let history = history.clone();
            l.create_function(
                move |lua,
                      (dst, ipoint, template_str, params): (
                    PathBuf,
                    Option<String>,
                    String,
                    Table,
                )| {
                    // Insertion point builder
                    let lookup = match ipoint {
                        Some(e) => format!("{}:{}", LUA_WEFTER_TEMPLATE_EMBEDDING_POINT, e),
                        None => format!("{}", LUA_WEFTER_TEMPLATE_EMBEDDING_POINT),
                    };
                    let params = serialize_table(lua, params)?;

                    log::debug!(
                        "[wefter.template.embed] inline template into {:?} at {:?}",
                        dst,
                        lookup
                    );

                    // Add contents to file
                    templates::embed_inline(dst.clone(), lookup.clone(), template_str, params)?;
                    history
                        .borrow_mut()
                        .push(HistoryAction::ModifyFile(dst, lookup));

                    Ok(())
                },
            )?
        }),
        /* @wefter.embed:template */
    ])
}
