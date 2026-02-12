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

-- ### FileSystem ### --

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

--- Read a file and get its content as a string.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path.
---
--- @return string|nil
---     File contents as string.
--- @return string|nil
---     IO Error, file does not exist?.
function loom.fs.read_to_string(path) end

--- Get a list of all items within a directory.
---
--- @param path string
---     Absolute or relative (to project root) path to directory.
---
--- @return table|nil
---     Array with paths to items in directory.
--- @return string|nil
---     IO Error, directory does not exist?.
function loom.fs.read_dir(path) end

-- ### I/O ### --

--- TUI related I/O.
---@class loom.io
loom.io = {}

--- Prompt user to input a string
---
--- @param prompt string
---     Message to show on the input prompt
---
--- @return string|nil
---     User input, or nil if none was given
function loom.io.input(prompt, opts) end

--- Prompt user to choose from a range of options, analogous to html <select>
---
--- @param prompt string
---     Message to show on the input prompt
---
--- @param opts table
---     Array with options as strings
---
--- @return string|nil
---     Selected option, or nil if none was selected
function loom.io.select(prompt, opts) end
