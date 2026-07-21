--- @meta

-- ### Constants ## --

--- Wefter version string.
--- @type string
--- @readonly
WEFTER_VERSION = ""

--- Absolute path to the project directory.
--- @type string
--- @readonly
WEFTER_PROJECT_ROOT = ""

--- Main Wefter API namespace exposed by the embedded Lua runtime.
---
--- This table is provided by the host Rust runtime and exists
--- at runtime without being required or imported.
--- @class wefter
wefter = {}

-- ### FileSystem ### --

--- Filesystem utilities. (early-loading)
--- This module is available during `auto.lua` and `init.lua` parsing
---
--- @class wefter.fs
wefter.fs = {}

--- Check whether a path exists and is a directory.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path
---
--- @return boolean
---     `true` if the path exists and is a directory, `false` otherwise.
function wefter.fs.is_dir(path) end

--- Check whether a path exists and is a regular file.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path
---
--- @return boolean
---     `true` if the path exists and is a file, `false` otherwise.
function wefter.fs.is_file(path) end

--- Read a file and get its content as a string.
---
--- @param path string
---     Absolute or relative (to project root) filesystem path.
---
--- @return string|nil
---     File contents as string.
--- @return string|nil
---     IO Error, file does not exist?.
function wefter.fs.read_to_string(path) end

--- Get a list of all items within a directory.
---
--- @param path string
---     Absolute or relative (to project root) path to directory.
---
--- @return table|nil
---     Array with paths to items in directory.
--- @return string|nil
---     IO Error, directory does not exist?.
function wefter.fs.read_dir(path) end

-- @wefter.embed:fs

-- ### I/O ### --

--- TUI related I/O.
--- @class wefter.io
wefter.io = {}

--- Prompt user to input a string
---
--- @param prompt string
---     Message to show on the input prompt
---
--- @return string
---     User input, fails if no input is given
function wefter.io.input(prompt) end

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
function wefter.io.select(prompt, opts) end

--- Render a markdown string into terminal.
---
--- @param content string
---     Some foo bar
--- 
--- @return nil
---     None. terminates program on error, use `pcall` if required.
function wefter.io.markdown(content) end

-- @wefter.embed:io

-- ### Templates ### --

--- Templating system API.
--- @class wefter.template
wefter.template = {}

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
function wefter.template.create(destination, template, params) end

--- Append contents of a template into an already existing file.
---
--- Create insertion points in file by creating a comment with the contents:
---     `@wefter.embed` or `@wefter.embed:<named>`
---
--- Contents will be appended before an _insertion point_, these are
--- comment lines on the destination file that contain the following string
--- `@wefter.embed:<ipoint>`, use parameter `ipoint` to specify multiple or
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
---         "foo" will insert before every "@wefter.embed:foo"
---         nil will insert at every "@wefter.embed:*"
---
---     You can also specify an unique insertion point "@wefter.embed" and
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
function wefter.template.embed(destination, ipoint, template, params) end

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
function wefter.template.get(template, params) end

-- @wefter.embed:template

--- Text manipulation utilities
--- @class wefter.txt
wefter.txt = {}

--- Transform a text into `snake_case`.
--- i.e "foo_bar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_snake_case(str) end

--- Transform a text into `constant_case`.
--- i.e "FOO_BAR"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_constant_case(str) end

--- Transform a text into `ada_case`.
--- i.e "Foo_Bar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_ada_case(str) end

--- Transform a text into `camel_case`.
---	i.e "fooBar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_camel_case(str) end

--- Transform a text into `upper_camel_case`.
--- i.e "FooBar"
---
--- Alias to `to_pascal_case`
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_upper_camel_case(str) end

--- Transform a text into `pascal_case`.
--- i.e "FooBar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_pascal_case(str) end

--- Transform a text into `kebab_case`.
--- i.e "foo-bar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_kebab_case(str) end

--- Transform a text into `train_case`.
--- i.e "Foo-Bar"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_train_case(str) end

--- Transform a text into `cobol_case`.
--- i.e "FOO-BAR"
---
--- @param str string
---     String to apply casing to
--- 
--- @return string
---     String with casing applied
function wefter.txt.to_cobol_case(str) end

-- @wefter.embed:txt
