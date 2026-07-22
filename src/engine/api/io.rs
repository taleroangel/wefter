use super::*;
use crate::tui::TuiInterface;
use std::rc::Rc;

/// Create a table for the 'io' submodule
pub fn module(l: &Lua, tui: Rc<TuiInterface>) -> Result<WefterModuleTable<'_>> {
    Ok(vec![
        // Prompt user to input a string
        ("input", {
            let tui = tui.clone();
            l.create_function(move |_, prompt: String| Ok(tui.input(prompt)?))?
        }),
        // Prompt user to choose from a selection
        ("select", {
            let tui = tui.clone();
            l.create_function(move |_, (prompt, opts): (String, Vec<String>)| {
                Ok(tui.select(&prompt, &opts)?)
            })?
        }),
        ("int", {
            let tui = tui.clone();
            l.create_function(
                move |_, (prompt, min, max): (String, Option<i32>, Option<i32>)| {
                    Ok(tui.integer(&prompt, min.unwrap_or(i32::MIN), max.unwrap_or(i32::MAX)))
                },
            )?
        }),
        ("confirm", {
            let tui = tui.clone();
            l.create_function(move |_, prompt: String| {
                Ok(tui.confirm(&prompt))
            })?
        }),
        ("markdown", {
            let tui = tui.clone();
            l.create_function(move |_, content: String| Ok(tui.print_markdown(content)))?
        }),
        /* @wefter.embed:io */
    ])
}
