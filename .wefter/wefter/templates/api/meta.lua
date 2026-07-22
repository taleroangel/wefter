--- {{ description }}.
---
{%- for param in params %}
--- @param {{ param.name }} {{ param.type.lua }}
---		{{ param.description }}
---
{%- endfor %}
--- @return {{ ret.type.lua }}
---     {{ ret.description }}
function wefter.{{ module }}.{{ name }}({% for param in params %}{{ param.name }}{% if not loop.last %}, {% endif %}{% endfor %}) end
