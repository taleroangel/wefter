# 🧵 Wefter
**Wefter** is a profile-driven automation runtime that lets you use Lua to define _cli_ command trees that perform structured, template-based code generation, embedding, or introspection.

In simple terms, **Wefter** helps you automate code scaffolding and project workflows:

* **Create custom CLI commands**: Define project-specific commands and subcommands using simple Lua scripts.
* **Generate files from templates**: Render new code files, components, or boilerplate using dynamic templates (powered by Tera/Jinja2).
* **Inject code into existing files**: Automatically insert imports, initialization code, or snippets into existing files at specified comment markers (`@wefter.embed`).
* **Prompt for user input**: Interactively ask for text input or menu selections right in the terminal.
* **Display rich Markdown in terminal**: Render formatted Markdown in the CLI for documentation, introspection, or command output.

## Quickstart 🚀

Get up and running with **Wefter** in a few steps by creating a project-local profile:

#### 1. Installation

Install Wefter directly from [crates.io](https://crates.io/crates/wefter):

```bash
cargo install wefter
```

Or install from source:

```bash
cargo install --path .
```

#### 2. Set up a project-local profile

Create a `.wefter` directory in the project root with a profile folder:

```bash
mkdir -p .wefter/my-profile/templates
```

#### 3. Add `auto.lua` & `init.lua`

Create `.wefter/my-profile/auto.lua` so Wefter automatically activates this profile when inside the project directory:

```lua
-- .wefter/my-profile/auto.lua
return true
```

Create `.wefter/my-profile/init.lua` to define custom commands:

```lua
-- .wefter/my-profile/init.lua
return {
  component = {
    description = "Component generation commands",
    subcommand = {
      create = {
        description = "Create a new UI component",
        exec = function()
          local name = wefter.io.input("Component name (e.g., Button)")
          wefter.template.create(
            "src/components/" .. name .. ".js",
            "component.js",
            { name = name }
          )
        end,
      },
    },
  },
}
```

#### 4. Create a template

Create `.wefter/my-profile/templates/component.js`:

```javascript
// .wefter/my-profile/templates/component.js
export function {{ name }}() {
  return <div>{{ name }} Component</div>;
}
```

#### 5. Run the command

Execute the CLI command:

```bash
wefter component create
```

Wefter will prompt for the component name, render the template, and create `src/components/Button.js`

### Examples

Check more advanced use cases in the `examples` directory, you can also check this repository's `.wefter` directory.

##  Profiles 📑

A **profile** is a self-contained module written in Lua that defines a CLI command tree and provides the templates to use.

Profile directory structure:

```
my-profile/
  init.lua
  auto.lua (optional)
  templates/
```

### `init.lua`

`init.lua` must return a Lua table describing the command hierarchy.

Example:

```lua
return {
  -- Top level command
  widget = {
    description = "Widget utilities",
    subcommand = {
      -- Subcommand within widget command
      create = {
        description = "Create a widget",
        exec = function()
          local name = wefter.io.input("Widget name")
          -- Create a new file from a template
          wefter.template.create(
            -- New file name (parent directories will be created)
            "lib/widgets/" .. name .. ".dart",
            -- Template path from profile (<profile>/templates/widget.dart)
            "widget.dart",
            -- Template parameters
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

A command can only have *subcommand* or *exec*

### `templates/`

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

### `auto.lua`

Helps `wefter` decide _which **profile** to use_ for the current project. If no `auto.lua` is specified, then the profile must be explicitly specified using `wefter -p my-profile`.

`auto.lua` is executed for every profile (when no profile is specified with `-p`). This file must return a boolean (`true` if the profile applies, `false` otherwise). Only one profile must be valid at a time; Wefter will request explicit profile selection on conflict.

In `auto.lua`, only `early-loading` API modules (such as `wefter.fs`) are available. It does not have access to other profile-specific modules or templates.

Here's a simple example of detecting a profile for a Rust application:

```lua
return wefter.fs.is_file("Cargo.toml")
```

For most project specific profiles ([.wefter directory](#profile-locations)) a simple `return true` statement is enough.

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

## Templates 📐

Wefter uses [Tera](https://keats.github.io/tera/docs/) as its template rendering engine. Tera is a powerful templating engine inspired by Jinja2 and Django templates.

### Overview

Templates live in your profile's `templates/` directory and support dynamic expression substitution (`{{ variable }}`), control flow constructs (`{% if %}`, `{% for %}`), filters, and formatting logic.

You can interact with templates using the Lua API:

- **`wefter.template.create(destination, template_path, params)`**: Renders a template and creates a new file at `destination`.
- **`wefter.template.embed(destination, insertion_point, template_path, params)`**: Renders a template and inserts its content into an existing file right before matching `@wefter.embed` or `@wefter.embed:<named>` insertion point comments.
- **`wefter.template.get(template_path, params)`**: Renders a template and returns the resulting content as a string.

You can also use _inline_ variant (create_inline, embed_inline) to use template strings instead of template paths.

For full details on template syntax, variables, filters, and control structures, check the official [Tera Documentation](https://keats.github.io/tera/docs/).

## Insertion Points 🪡

**Insertion points** allow Wefter to inject dynamically rendered code directly into existing files at specific locations marked by special comments.

### Comment Syntax

Place an insertion point comment anywhere in your target file using your language's standard comment syntax (`//`, `/* ... */`, `#`, `--`, `<!-- ... -->`):

- **Default Insertion Point**: `@wefter.embed` (matches when `ipoint` is `nil`)
- **Named Insertion Point**: `@wefter.embed:<name>` (e.g., `@wefter.embed:includes`, `@wefter.embed:routes`)

```c
// main.c
#include <stdio.h>

/* @wefter.embed:includes */

int main(void) {
    /* @wefter.embed:init */

    return 0;
}
```

### Embedding via Lua API

Use `wefter.template.embed` or `wefter.template.embed_inline` to insert content at an insertion point:

- **`wefter.template.embed(destination, ipoint, template_path, params)`**: Renders a template file and inserts it.
- **`wefter.template.embed_inline(destination, ipoint, template_str, params)`**: Renders an inline template string and inserts it.

#### Parameters:
* `destination` *(string)*: Target file path relative to project root.
* `ipoint` *(string | nil)*: Target insertion point name (e.g., `"includes"` matches `@wefter.embed:includes`). Pass `nil` to target generic `@wefter.embed` comments.
* `template` / `template_str` *(string)*: Template file path inside `templates/` or raw template string.
* `params` *(table)*: Parameters passed to the Tera rendering engine.

### Example

In your Lua profile script (`init.lua`):

```lua
-- Inject an include statement at `@wefter.embed:includes`
wefter.template.embed_inline("main.c", "includes", '#include "{{ header }}"', { header = "module.h" })

-- Inject initialization code at `@wefter.embed:init`
wefter.template.embed_inline("main.c", "init", 'init_module("{{ name }}");', { name = "my_module" })
```

#### Resulting `main.c`:

```c
#include <stdio.h>

#include "module.h"
/* @wefter.embed:includes */

int main(void) {
    init_module("my_module");
    /* @wefter.embed:init */

    return 0;
}
```

### Key Behaviors

* **Automatic Indentation Matching**: Injected code automatically matches the indentation (leading spaces or tabs) of the insertion point comment line.
* **Marker Preservation**: The insertion point comment remains in place after injection, allowing subsequent commands to insert additional code at the same marker.
* **Multiple Markers**: If multiple matching insertion point comments exist in the target file, content is injected before each occurrence.

## Lua API ⚒️

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

## Lua LSP 🧰

Need help navigating the API? _wefter_ can create a [Lua Definition File](https://luals.github.io/wiki/definition-files/), this file will be used by your LSP to give you proper diagnostics and completion.

Create a **definition file** using the following command:
```
wefter --meta > .wefter.d.lua
```

Then, setup the LSP to use it.

**LuaLS** example, `.luarc.json`:
```json
{
    "workspace.library": [".wefter.d.lua"],
    "diagnostics.globals": ["wefter", "WEFTER_VERSION", "WEFTER_PROJECT_ROOT"]
}
```

## Acknowledgements ❤️

Wefter is built on top of amazing open-source libraries in the Rust ecosystem:

* [**mlua**](https://crates.io/crates/mlua) - Embedded Lua runtime bindings for Rust.
* [**tera**](https://crates.io/crates/tera) - Powerful Jinja2/Django-inspired templating engine.
* [**clap**](https://crates.io/crates/clap) - Command line argument parsing and help formatting.
* [**inquire**](https://crates.io/crates/inquire) - Interactive terminal prompts for user inputs and selections.
* [**termimad**](https://crates.io/crates/termimad) - In-terminal Markdown rendering engine.
* [**directories**](https://crates.io/crates/directories) - Cross-platform path resolution for system configuration and data folders.
* [**convert_case**](https://crates.io/crates/convert_case) - String case conversion helpers (_wefter.txt_ API).

## AI Notice 🤖

The core codebase of Wefter is mostly human-written (90%+). AI assistance was brealy used and was mainly focused on documentation and minor boilerplate

