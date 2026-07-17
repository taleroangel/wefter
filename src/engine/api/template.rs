use super::*;
use crate::{ResourceDir, fs::hist::{History, HistoryAction}, templates};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

/// String for embedding into files
const LUA_LOOM_TEMPLATE_EMBEDDING_POINT: &str = "@loom.embed";

// Create a table for the 'template' submodule
pub fn module(l: &Lua, profile: ResourceDir, history: Rc<RefCell<History>>) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        ("create", {
            let profile = profile.clone();
            let history = history.clone();
            l.create_function(
                move |lua, (dst, template, params): (PathBuf, PathBuf, Table)| {
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;
                    log::debug!(
                        "[loom.template.create] Creating file {:?} with template {:?}",
                        dst,
                        template
                    );

                    // Get file contents
                    let rendered = templates::render(template, params)?;
                    
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
                move |lua, (dst, ipoint, template, params): (
                    PathBuf,
                    Option<String>,
                    PathBuf,
                    Table,
                )| {
                    // Insertion point builder
                    let lookup = match ipoint {
                        Some(e) => format!("{}:{}", LUA_LOOM_TEMPLATE_EMBEDDING_POINT, e),
                        None => format!("{}", LUA_LOOM_TEMPLATE_EMBEDDING_POINT),
                    };
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;

                    log::debug!(
                        "[loom.template.embed] template {:?} into {:?} at {:?}",
                        template,
                        dst,
                        lookup
                    );
                    
                    // Add contents to file
                    templates::embed(dst.clone(), lookup.clone(), template, params)?;
                    history.borrow_mut().push(HistoryAction::ModifyFile(dst, lookup));

                    Ok(())
                },
            )?
        }),
        ("get", {
            let profile = profile.clone();
            l.create_function(move |lua, (template, params): (PathBuf, Table)| {
                let template = profile.build_template_path(template)?;
                let params = serialize_table(lua, params)?;

                log::debug!("[loom.template.get] template {:?}", template);
                let rendered = templates::render(template, params)?;
                Ok(rendered)
            })?
        }),
        /* @loom.embed:template */
    ])
}
