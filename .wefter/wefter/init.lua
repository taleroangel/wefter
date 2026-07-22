---@type wefter.profile
return {
	--- Commands related to the Wefter API
	api = {
		description = "Utilities for creating/listing functions in the Wefter api",
		subcommand = {
			--- Create a new API entry
			create = {
				description = "Create a new function within a module",
				exec = function()
					-- Read modules in API directory
					local dir, err = wefter.fs.read_dir("src/engine/api");
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

					-- Parameters type (Lua <-> Rust)
					local ptypes = {
						["integer (signed)"] = { rust = "i32", lua = "integer" },
						["integer (unsigned)"] = { rust = "u32", lua = "integer" },
						["boolean"] = { rust = "bool", lua = "boolean" },
						["string"] = { rust = "String", lua = "string" },
						["string (path)"] = { rust = "PathBuf", lua = "string" },
						["table"] = { rust = "Table", lua = "table" },
						["function"] = { rust = "Function", lua = "function" },
						["any"] = { rust = "Any", lua = "any" }
					}

					-- Get parameter type as user options
					local ptypes_options = {}
					for type, _ in pairs(ptypes) do
						table.insert(ptypes_options, type)
					end

					-- Show all API modules in a list and let the user pick
					local mod = wefter.io.select("Select a module", entries)

					-- Get function parameters as text from the user
					local name = wefter.io.input("Name for the function")
					local desc = wefter.io.input("Function description")

					-- Get return data	
					local ret = {
						description = wefter.io.input("Function return description"),
						type = ptypes[wefter.io.select("Function return type", ptypes_options)]
					}

					-- Get function parameters
					local n_params = wefter.io.int("Number of parameters", 0)
					local params = {};
					for i = 1, n_params do
						-- Ask for parameter name
						local pname = wefter.io.input("Parameter (" .. i .. ") name")
						-- Ask for parameter type
						local ptype = wefter.io.select("Parameter (" .. i .. ") type", ptypes_options)
						-- Ask for parameter description
						local pdesc = wefter.io.input("Parameter (" .. i .. ") description")
						-- Insert into parameters table
						table.insert(params, { name = pname, type = ptypes[ptype], description = pdesc })
					end

					-- Insert contents of template `templates/api/meta.lua`
					-- into file `static/lua/wefter.d.lua`
					-- at the insertion point `@wefter.embed:<mod>`
					wefter.template.embed("static/lua/wefter.d.lua", mod, "api/meta.lua", {
						module = mod,
						description = desc,
						name = name,
						params = params,
						ret = ret
					})

					-- Insert contents of template `templates/api/api.rs`
					-- into file `src/engine/api/<mod>.rs`
					-- at the insertion point `@wefter.embed:<mod>`
					wefter.template.embed(
						"src/engine/api/" .. mod .. ".rs",
						mod,
						"api/api.rs",
						{
							module = mod,
							name = name,
							params = params,
							ret = ret,
						})
				end,
			},
			--- List all of the API entries
			list = {
				exec = function()
					-- Read modules in API directory
					local dir, err = wefter.fs.read_dir("src/engine/api");
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
					local template = wefter.template.get("api/list.md", { items = entries })

					-- Render markdown to screen
					wefter.io.markdown(template)

					wefter.template.create("foo/bar/destination.rs", "api/api.rs", { module = "foo", name = "bar" })
				end
			}
		},
	},
}
