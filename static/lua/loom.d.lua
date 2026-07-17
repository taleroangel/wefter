--- @meta

-- ### Constants ## --

--- Loom version string.
--- @type string
--- @readonly
LOOM_VERSION = ""

--- Absolute path to the project directory.
--- @type string
--- @readonly
LOOM_PROJECT_ROOT = ""

--- Main Loom API namespace exposed by the embedded Lua runtime.
---
--- This table is provided by the host Rust runtime and exists
--- at runtime without being required or imported.
--- @class loom
loom = {}

-- ### FileSystem ### --

--- Filesystem utilities. (early-loading)
--- This module is available during `auto.lua` and `init.lua` parsing
---
--- @class loom.fs
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

-- @loom.embed:fs

-- ### I/O ### --

--- TUI related I/O.
--- @class loom.io
loom.io = {}

--- Prompt user to input a string
---
--- @param prompt string
---     Message to show on the input prompt
---
--- @return string
---     User input, fails if no input is given
function loom.io.input(prompt) end

--- Prompt user to choose from a range of options, analogous to html <select>
---
--- @param prompt string
---     Message to show on the input prompt
---
--- @param opts table
---     Array with options as strings
---
--- @return string
---     Selected option, fails if no option was selected
function loom.io.select(prompt, opts) end

--- Render a markdown string into terminal.
---
--- @param content string
---     Some foo bar
--- 
--- @return nil
---     None. terminates program on error, use `pcall` if required.
function loom.io.markdown(content) end

-- @loom.embed:io

-- ### Templates ### --

--- Templating system API.
--- @class loom.template
loom.template = {}

--- Create a new file from a given template.
---
--- @param destination string
---     New file absolute or relative (to project root)
---
--- @param template string
---     Template path (must be relative to profile `templates` directory)
---     i.e "foo/bar.txt" resolves to "{profileDir}/foo/bar.txt"
---
--- @param params table
---     json-like parameters for the template
---
--- @return nil
---     None. terminates program on error, use `pcall` if required.
function loom.template.create(destination, template, params) end

--- Append contents of a template into an already existing file.
---
--- Create insertion points in file by creating a comment with the contents:
---     `@loom.embed` or `@loom.embed:<named>`
---
--- Contents will be appended before an _insertion point_, these are
--- comment lines on the destination file that contain the following string
--- `@loom.embed:<ipoint>`, use parameter `ipoint` to specify multiple or
--- distinct insertion points, if `ipoint` is not specified (nil), then
--- the template will be inserted before all of the insertion points.
---
--- @param destination string
---     Filepath absolute or relative (to project root)
---
--- @param ipoint string|nil
---     Insertion point specifier, or nil to use them all!
---
---     i.e
---         "foo" will insert before every "@loom.embed:foo"
---         nil will insert at every "@loom.embed:*"
---
---     You can also specify an unique insertion point "@loom.embed" and
---     keep this parameter nil
---
--- @param template string
---     Template path (must be relative to profile `templates` directory)
---
---     i.e "foo/bar.txt" resolves to "{profileDir}/foo/bar.txt"
---
--- @param params table
---     json-like parameters for the template
---
--- @return nil
---     None. terminates program on error, use `pcall` if required.
function loom.template.embed(destination, ipoint, template, params) end

--- Render a template and get its contents as a string.
---
--- @param template string
---     Template path (must be relative to profile `templates` directory)
---
---     i.e "foo/bar.txt" resolves to "{profileDir}/foo/bar.txt"
---
--- @param params table
---     json-like parameters for the template
--- 
--- @return string
---     Rendered template contents. terminates program on error, use `pcall` if required.
function loom.template.get(template, params) end

-- @loom.embed:template

--- Text manipulation utilities
--- @class loom.txt
loom.txt = {}

-- @loom.embed:txt
