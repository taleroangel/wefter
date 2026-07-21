        ("{{ name }}", {
            l.create_function(move |lua, foo: Value| {
                log::debug!("[wefter.{{ module }}.{{ name }}]");
                Ok(())
            })?
        }),
