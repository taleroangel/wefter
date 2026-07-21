# Wefter
**Wefter** is a profile-driven automation runtime that lets you use Lua to define _cli_ command trees that perform structured, template-based code generation, embedding, or introspection.

## Profiles

A **profile** is a self-contained automation module that defines a CLI command tree in Lua and provides the templates it uses.

Profile directory structure:

```
my-profile/
  init.lua
  auto.lua (optional)
  templates/
```

#### `init.lua`

`init.lua` must return a Lua table describing the command hierarchy.

Example:

```lua
return {
  widget = {
    description = "Widget utilities",
    subcommand = {
      create = {
        description = "Create a widget",
        exec = function()
          local name = wefter.io.input("Widget name")

          -- Create a new file from a template
          wefter.template.create(
            -- New file name
            "lib/widgets/" .. name .. ".dart",
            -- Template name (profile/templates/widget.dart)
            "widget.dart",
            -- Widget parameters
            { name = name }
          )
        end,
      },
    },
  },
}
```

This creates the following command:

```
wefter widget create
```

Each node may contain:

* `description` - String describing the command (for showing help on bad usage)
* `subcommand` - Table with key, value pairs for other commands
* `exec` - Function to execute when the command is called

#### `templates/`

This directory contains all user-defined templates (visit [Templates](#templates) on how templates work and how to create them).

Templates are resolved relative to the profile’s `templates/` directory.

i.e.

```
my-profile/
    templates/
        foo/
            bar.md
        baz.md
```

```lua
-- Create a file `hello.md` from template `my-profile/templates/foo/bar.md`
wefter.template.create("hello.md", "foo/bar.md", {})
-- Get rendered template as string from `my-profile/templates/baz.md`
local baz = wefter.template.get("baz.md", {})

-- Render template to cli (wefter has an embedded markdown CLI renderer)
-- You can use this feature to build introspection commands (i.e list classes,
-- show dependencies, etc) and presenting them with your own markdown template
wefter.io.markdown(baz);
```

#### `auto.lua` (optional)

Helps `wefter` decide _which **profile** to use_ for the current project. If no `auto.lua` is specified, then the profile must be explicitly specified using `wefter -p my-profile`.

`auto.lua` is executed for every profile (when no profile is specified with `-p`). This file must return a boolean (`true` if the profile applies, `false` otherwise). Only one profile must be valid at a time; Wefter will request explicit profile selection on conflict.

In `auto.lua`, only `early-loading` API modules (such as `wefter.fs`) are available. It does not have access to other profile-specific modules or templates.

Here's a simple example of detecting a profile for a Rust application:

```lua
return wefter.fs.is_file("Cargo.toml")
```

### Profile Locations

Profiles can be defined either system-wide or per project.

**System profiles** live in the configured data directory, `~/.local/share/wefter` by default, but location can be changed in the configuration file `~/.config/wefter/wefter.toml`:

```toml
data_dir = "/home/<user>/.local/share/wefter"
```

Paths vary by OS and are resolved using the Rust [directories](https://crates.io/crates/directories) crate.

**Project-local profiles** live inside a folder named `.wefter` in the project root directory.
```
my-project/
    src/
    .wefter/ (Project-local profiles)
        my-profile/
```

Use the data directory for reusable profiles, and `.wefter/` for project-specific ones.

## Templates

Wefter uses [Tera](https://keats.github.io/tera/docs/) as its template rendering engine. Tera is a powerful templating engine inspired by Jinja2 and Django templates.

### Overview

Templates live in your profile's `templates/` directory and support dynamic expression substitution (`{{ variable }}`), control flow constructs (`{% if %}`, `{% for %}`), filters, and formatting logic.

You can interact with templates using the Lua API:
* **`wefter.template.create(destination, template_path, params)`**: Renders a template and creates a new file at `destination`.
* **`wefter.template.embed(destination, insertion_point, template_path, params)`**: Renders a template and inserts its content into an existing file right before matching `@wefter.embed` or `@wefter.embed:<named>` insertion point comments.
* **`wefter.template.get(template_path, params)`**: Renders a template and returns the resulting content as a string.

For full details on template syntax, variables, filters, and control structures, check the official [Tera Documentation](https://keats.github.io/tera/docs/).

## Lua API

Wefter exposes a global `wefter` namespace in the Lua runtime along with environment constants:

### Global Constants
* `WEFTER_VERSION`: Current Wefter version string.
* `WEFTER_PROJECT_ROOT`: Absolute path to the root directory of the current project.

### API Modules
* **`wefter.fs`**: Filesystem utilities (`is_dir`, `is_file`, `read_to_string`, `read_dir`). Available during early-loading (`auto.lua`).
* **`wefter.io`**: TUI and user interaction tools (`input`, `select`, `markdown`).
* **`wefter.template`**: Template rendering and code generation utilities (`create`, `embed`, `get`).
* **`wefter.txt`**: String casing transformation helpers (`to_snake_case`, `to_camel_case`, `to_pascal_case`, `to_upper_camel_case`, `to_kebab_case`).

For full type definitions, function signatures, and docstrings, see the definition file [`static/lua/wefter.d.lua`](static/lua/wefter.d.lua) (or generate it locally via `wefter --meta > .wefter.d.lua`).

## Lua LSP

Need help navigating the API? _wefter_ can create a [Lua Definition File](https://luals.github.io/wiki/definition-files/), this file will be used by your LSP to give you proper diagnostics and completion.

Create a **definition file** using the following command:
```
wefter --meta > .wefter.d.lua
```

Then, setup your favorite LSP to use it.

**LuaLS** example, `.luarc.json`:
```json
{
    "workspace.library": [".wefter.d.lua"],
    "diagnostics.globals": ["wefter", "WEFTER_VERSION", "WEFTER_PROJECT_ROOT"]
}
```

