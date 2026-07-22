#ifndef __{{ header_guard }}_H__
#define __{{ header_guard }}_H__

/**
 * @class {{ class_name }}
 * @brief {{ class_description }}
 */
struct {{ class_name }} {
	{%- for member in members %}
	{{ member.type }} {{ member.name }};
	{%- endfor %}
};

#endif // __{{ header_guard }}_H__
