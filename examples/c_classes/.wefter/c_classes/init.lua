return {
	class = {
		description = "Command to manipulate classes",
		subcommand = {
			new = {
				description = "Create a new class",
				exec = function()
					-- Get class name
					local class = wefter.io.input("Class name (in any casing, i.e. 'my class')")

					-- Get class description
					local description = wefter.io.input("Class description")

					-- Class name in different casing styles
					local class_snake_case = wefter.txt.to_snake_case(class) -- my_class
					local class_constant_case = wefter.txt.to_constant_case(class) -- MY_CLASS
					local class_pascal_case = wefter.txt.to_pascal_case(class) -- MyClass
					local class_camel_case = wefter.txt.to_camel_case(class) -- myClass

					-- Ask for member variables
					local members = {}
					local n_members = wefter.io.int("How many member variables to add?", 0)
					for i = 1, n_members do
						-- For every member as for its name and type
						local member_name = wefter.io.input("Variable (" .. i .. ") name")
						local member_type = wefter.io.select("Variable (" .. i .. ") type", {
							"char",
							"short",
							"int",
							"long",
							"float",
							"double",
							"char *",
							"void *"
						})

						-- Append variable
						table.insert(members, { type = member_type, name = member_name })
					end

					-- Name for the .h file
					local class_file = class_snake_case .. ".h"
					-- Object file name
					local object_file = class_snake_case .. ".o"
					-- Class name in upper camel case (pascal case)
					local class_name = class_pascal_case
					-- Variable name in lower camel case
					local variable_name = class_camel_case

					-- Ask for user confirmation
					local continue = wefter.io.confirm(
						"Create a class `" .. class_name ..
						"` at `" .. class_file ..
						"` and heap variable `" .. variable_name ..
						"` in `main.c`")
					if not continue then
						return
					end

					-- Create .h file from template `templates/class.h`
					wefter.template.create(class_file, "class.h", {
						header_guard = class_constant_case,
						class_name = class_name,
						class_description = description,
						members = members
					})

					-- Include newly created .h file in main.c at `@wefter.embed:include`
					wefter.template.embed_inline("main.c", "include", '#include "{{ file }}"', { file = class_file })

					-- Create a heap variable for the newly created class in main.c at `@wefter.embed:malloc`
					wefter.template.embed_inline(
						"main.c",
						"malloc",
						"struct {{ class_name }} * {{ var_name }} = (struct {{ class_name }} *)malloc(sizeof(struct {{ class_name }}));",
						{
							class_name = class_name,
							var_name = variable_name,
						}
					)

					-- Free heap memory for the variable created before at `@wefter.embed:free`
					wefter.template.embed_inline(
						"main.c",
						"free",
						"free(" .. variable_name .. ");",
						{}
					)
				end
			}
		}
	}
}
