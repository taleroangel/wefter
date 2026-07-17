return {
	api = {
		description = "Utilities for creating/listing functions in the Loom api",
		subcommand = {
            -- Create a new API entry
			create = {
				description = "Create a new function within a module",
				exec = function()
					-- Get module to append to
					local mod = loom.io.select("Select a module", {
						"fs",
						"io",
						"template",
					})

					-- Get function parameters
					local name = loom.io.input("Name for the function")
					local desc = loom.io.input("Function description")

					-- Insert into `loom.d.lua`
					loom.template.embed("static/lua/loom.d.lua", mod, "api/meta.lua", {
						module = mod,
						description = desc,
						name = name,
					})

					-- Insert into `engine/api.rs`
					loom.template.embed(
						"src/engine/api/" .. mod .. ".rs",
						mod,
						"api/api.rs",
					{
						module = mod,
						name = name,
					})
				end,
			},
		},
	},
}
