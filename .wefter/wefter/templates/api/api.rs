("{{ name }}", {
    l.create_function(move |lua, ({% for param in params %}{{ param.name }}{% if not loop.last %}, {% endif %}{% endfor %}): ({% for param in params %}{{ param.type.rust }}{% if not loop.last %}, {% endif %}{% endfor %})| {
        log::debug!("[wefter.{{ module }}.{{ name }}]");
        Ok(())
    })?
}),
