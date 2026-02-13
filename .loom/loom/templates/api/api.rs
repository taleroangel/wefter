        ("{{ name }}", {
            l.create_function(move |lua, foo: Value| {
                log::debug!("[loom.{{ module }}.{{ name }}]");
                Ok(())
            })?
        }),
