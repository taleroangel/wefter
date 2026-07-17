return {
	--- Commands related to the Loom API
	api = {
		description = "Utilities for creating/listing functions in the Loom api",
		subcommand = {
			--- Create a new API entry
			create = {
				description = "Create a new function within a module",
				exec = function()
					-- Read modules in API directory
					local dir, err = loom.fs.read_dir("src/engine/api");
					if err then
						error(err)
					end
					---@cast dir table

					-- Modules names are stored here
					local entries = {}
					for _, entry in ipairs(dir) do
						local filename = entry:match("([^/]+)%.rs$")
						if filename ~= "mod" then
							table.insert(entries, filename)
						end
					end

					-- Show all API modules in a list and let the user pick
					local mod = loom.io.select("Select a module", entries)

					-- Get function parameters as text from the user
					local name = loom.io.input("Name for the function")
					local desc = loom.io.input("Function description")

					-- Insert contents of template `templates/api/meta.lua`
					-- into file `static/lua/loom.d.lua`
					-- at the insertion point `@loom.embed:<mod>`
					loom.template.embed("static/lua/loom.d.lua", mod, "api/meta.lua", {
						module = mod,
						description = desc,
						name = name,
					})

					-- Insert contents of template `templates/api/api.rs`
					-- into file `src/engine/api/<mod>.rs`
					-- at the insertion point `@loom.embed:<mod>`
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
			--- List all of the API entries
			list = {
				exec = function()
					-- Read modules in API directory
					local dir, err = loom.fs.read_dir("src/engine/api");
					if err then
						error(err)
					end
					---@cast dir table

					-- Modules names are stored here
					local entries = {}
					for _, entry in ipairs(dir) do
						if entry ~= "src/engine/api/mod.rs" then
							local name = entry:gsub("%.rs$", "")
							table.insert(entries, name)
						end
					end

					-- Get API markdown template `emplates/api/list.md`
					local template = loom.template.get("api/list.md", { items = entries })

					-- Render markdown to screen
					loom.io.markdown(template)
				end
			}
		},
	},
}
