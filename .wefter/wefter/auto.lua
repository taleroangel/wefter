-- Cargo.toml exists
if not wefter.fs.is_file("Cargo.toml") then
	return false
end

-- Read Cargo.toml
local str, _ = wefter.fs.read_to_string("Cargo.toml")
if not str then
	return false
end

-- Contains `name = "wefter"`
return (string.find(str, 'name = "wefter"') ~= nil)
