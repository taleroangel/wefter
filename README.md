# Loom
**Loom** is a profile-driven automation runtime that lets you use Lua to define _cli_ command trees that perform structured, template-based code generation, embedding, or introspection.

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
          local name = loom.io.input("Widget name")

          -- Create a new file from a template
          loom.template.create(
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
loom widget create
```

Each node may contain:

* `description` - String describing the command (for showing help on bad usage)
* `subcommand` - Table with key, value pairs for other commands
* `exec` - Function to execute when the command is called

#### `templates/`

This directory contains all the user-defined templates (visit [templates](#templates) on how to use & create templates)

Templates are resolved relative to the profile’s `templates/` directory.

i.e

```
my-profile/
    templates/
        foo/
            bar.md
        baz.md
```

```lua
-- Create a file `hello.md` from template `my-profile/templates/foo/bar.md`
loom.template.create("hello.md", "foo/bar.md", {})
-- Get rendered template as string from `my-profile/templates/baz.md`
local baz = loom.template.get("baz.md", {})

-- Render template to cli (loom has an embedded markdown CLI renderer)
-- You can use this feature to build introspection commands (i.e list classes,
-- show dependencies, etc) and presenting them with your own markdown template
loom.io.markdown(baz);
```

#### `auto.lua` (optional)

Helps `loom` decide _which **profile** to use_ for the current project, if no `auto.lua` is specified, then the profile must be explicitly specified using `loom -p my-profile`

`auto.lua` is executed for every profile (when no profile is specified with `-p`), this file must return a boolean with the value of `true` if the profile applies, `false` otherwise, and only one profile must be valid at a time, loom will request explicit profile selection on conflict.

Here's a simple example of detecting a _profile_ for a Rust application

```lua
return loom.fs.is_file("Cargo.toml")
```

### Profile Locations

Profiles can be defined either system-wide or per project.

**System profiles** live in the configured data directory, `~/.local/share/loom` by default, but location can be changed in the configuration file `~/.local/config/loom/loom.toml`:

```toml
data_dir = "/home/<user>/.local/share/loom"
```

Paths vary by OS and are resolved using the Rust [directories](https://crates.io/crates/directories) crate.

**Project-local profiles** live inside a folder named `.loom` in the project root directory.
```
my-project/
    src/
    .loom/ (Project-local profiles)
        my-profile/
```

Use the data directory for reusable profiles, and `.loom/` for project-specific ones.


### Template Engine (tera)
https://keats.github.io/tera/docs/

### auto.lua
Does not have access to other modules in its directory
