-- Cargo.toml exists
if not loom.fs.is_file("Cargo.toml") then
	return false
end

-- Read Cargo.toml
local str, _ = loom.fs.read_to_string("Cargo.toml")
if not str then
	return false
end

-- Contains `name = "loom"`
return (string.find(str, 'name = "loom"') ~= nil)
