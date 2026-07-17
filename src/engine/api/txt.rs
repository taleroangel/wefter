use convert_case::{Case, Casing};

use super::*;

pub fn module(l: &Lua) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        ("to_snake_case", {
            l.create_function(move |_, str: String| Ok(str.to_case(Case::Snake)))?
        }),
        ("to_camel_case", {
            l.create_function(move |_, str: String| Ok(str.to_case(Case::Camel)))?
        }),
        ("to_upper_camel_case", {
            l.create_function(move |_, str: String| Ok(str.to_case(Case::Pascal)))?
        }),
        ("to_pascal_case", {
            l.create_function(move |_, str: String| Ok(str.to_case(Case::Pascal)))?
        }),
        ("to_kebab_case", {
            l.create_function(move |_, str: String| Ok(str.to_case(Case::Kebab)))?
        }),
        /* @loom.embed:txt */
    ])
}
