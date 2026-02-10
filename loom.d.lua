--- @meta

-- ### Constants ## --

--- Loom version string.
---@type string
---@readonly
LOOM_VERSION = ""

--- Absolute path to the project directory.
---@type string
---@readonly
LOOM_PROJECT_ROOT = ""

--- Main Loom API namespace exposed by the embedded Lua runtime.
---
--- This table is provided by the host Rust runtime and exists
--- at runtime without being required or imported.
---@class loom
loom = {}

-- ### Filesystem ### --

--- Filesystem utilities.
---@class loom.fs
loom.fs = {}

--- Check whether a path exists and is a directory.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path
---
--- @return boolean
---     `true` if the path exists and is a directory, `false` otherwise.
function loom.fs.is_dir(path) end

--- Check whether a path exists and is a regular file.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path
---
--- @return boolean
---     `true` if the path exists and is a file, `false` otherwise.
function loom.fs.is_file(path) end
