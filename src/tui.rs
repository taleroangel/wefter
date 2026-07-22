use crate::{
    cli::Params,
    engine::{CommandMap, ProfileDef},
    error::WefterErr,
    fs::{self, dirs::DirCfg, hist::HistoryAction, res::ResourceDirTable},
};
use anyhow::Result;
use clap::CommandFactory;
use clap_help::Printer;
use termimad::{
    MadSkin,
    crossterm::style::Color,
    minimad::{OwningTemplateExpander, TextTemplate},
};

/// wefter.d.lua API documentation
const LUA_API_META: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/lua/wefter.d.lua"
));

/// Template for the help description
const HELP_TEMPLATE_MD: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/cli/about.md"));

/// Template for the help description
const PROFILE_DEF_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/cli/profile_def.md"
));

/// Template for showing resources
const PROFILE_LIST_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/cli/profile_list.md"
));

/// Markdown template for [WefterErr] generic error
const ERRORS_GENERIC_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/errors/generic_error.md"
));

/// Markdown template for [WefterErr::NoAvailableProfiles] error
const ERRORS_NO_AVAILABLE_PROFILES_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/errors/no_available_profiles.md"
));

/// Markdown template for [WefterErr::EmptyParameters] error
const ERRORS_EMPTY_PARAMETERS_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/errors/empty_parameters.md"
));

/// Interface for manipulating the TUI
pub struct TuiInterface {
    skin: MadSkin,
}

impl Default for TuiInterface {
    /// Initialize interface with default skin
    fn default() -> Self {
        // Build style
        let mut skin = MadSkin::default();
        skin.set_headers_fg(Color::Blue);
        skin.table.align = termimad::Alignment::Left;
        skin.bold.set_fg(Color::Blue);
        skin.italic.set_fg(Color::Magenta);

        Self { skin }
    }
}

impl TuiInterface {
    /// Alias for ::default
    pub fn new() -> Self {
        Self::default()
    }

    /// Print markdown directly to terminal
    pub fn print_markdown(&self, content: String) {
        self.skin.print_text(&content);
    }

    /// Print `wefter.d.lua` file
    pub fn print_lua_meta(&self) {
        println!("{}", LUA_API_META)
    }

    /// Print help message using Tui skin
    pub fn print_help(&self) {
        Printer::new(Params::command())
            .with("introduction", HELP_TEMPLATE_MD)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .with_skin(self.skin.clone())
            .print_help();
    }

    /// Pretty print a table with all available resources
    pub fn print_profile_list(&self, resources: &ResourceDirTable, dirs: &DirCfg) {
        // Use static markdown template
        let mdtemplate = TextTemplate::from(PROFILE_LIST_TEMPLATE_MD);
        let mut mdexpander = OwningTemplateExpander::new();

        // Show where the resources come from
        mdexpander.set("system-source", format!("{:?}", dirs.data));
        mdexpander.set(
            "local-source",
            if let Some(local) = &dirs.local {
                format!("{:?}", local)
            } else {
                format!("Not found")
            },
        );

        // Show properties for each resource
        for (k, v) in resources.iter() {
            // Push data to row template
            mdexpander
                .sub("resource-rows")
                .set("resource-profile", &k)
                .set("resource-path", format!("{:?}", &v.path))
                .set_md(
                    "resource-auto",
                    if v.auto.is_some() { "**✓**" } else { "*✗*" },
                );
        }

        // Print table
        self.skin.print_owning_expander(&mdexpander, &mdtemplate);
    }

    /// Show error
    pub fn print_error(&self, err: &WefterErr) {
        // Use static markdown template
        let mdtemplate = TextTemplate::from(ERRORS_GENERIC_TEMPLATE_MD);
        let mut mdexpander = OwningTemplateExpander::new();

        mdexpander.set("error-message", &err.to_string());
        println!(); // newline
        self.skin.print_owning_expander(&mdexpander, &mdtemplate);
    }

    /// Print documentation for [WefterErr::NoAvailableProfiles]
    pub fn print_err_no_available_profiles(&self, dirs: &DirCfg) {
        // Use static markdown template
        let mdtemplate = TextTemplate::from(ERRORS_NO_AVAILABLE_PROFILES_TEMPLATE_MD);
        let mut mdexpander = OwningTemplateExpander::new();

        mdexpander.set("error-message", WefterErr::NoAvailableProfiles.to_string());
        mdexpander.set("path", format!("{:?}", dirs.data));

        self.skin.print_owning_expander(&mdexpander, &mdtemplate);
    }

    /// Print documentation for [WefterErr::EmptyParameters]
    pub fn print_err_empty_parameters(&self, profile: &String, def: &ProfileDef) {
        // Use static markdown template
        let mdtemplate = TextTemplate::from(ERRORS_EMPTY_PARAMETERS_TEMPLATE_MD);
        let mut mdexpander = OwningTemplateExpander::new();

        mdexpander.set("error-message", WefterErr::EmptyParameters.to_string());

        self.skin.print_owning_expander(&mdexpander, &mdtemplate);

        // Now show the actual profile def
        self.print_profile(profile, def);
    }

    /// Print file history
    pub fn print_history(&self, history: fs::hist::History) {
        println!(); // newline
        self.skin.print_inline("**Files changed: **");
        println!(); // newline
        for action in history {
            self.skin.print_inline(
                match action {
                    HistoryAction::FileCreated(path) => {
                        format!("*[CREATED]* {:?}", path)
                    }
                    HistoryAction::FileModified(path, point) => {
                        format!("*[MODIFIED at {}]* {:?}", point, path)
                    }
                    HistoryAction::FileRenamed { previous, new } => {
                        format!("*[RENAMED]* file {:?} to {:?}", previous, new.file_name())
                    }
                    HistoryAction::FileMoved { previous, new } => {
                        format!("*[MOVED]* file {:?} to {:?}", previous, new)
                    }
                    HistoryAction::FileDeleted(path) => {
                        format!("*[DELETED]* {:?}", path)
                    }
                    HistoryAction::FileCopied { src, dst } => {
                        format!("*[COPIED]* {:?} into {:?}", src, dst)
                    }
                    HistoryAction::DirectoryCreated(path) => {
                        format!("*[CREATED]* (directory) {:?}", path)
                    }

                }
                .as_str(),
            );
            println!(); // newline
        }
    }

    /// Recursive function to print a subcommand tree
    fn render_subcommands(&self, out: &mut String, sub: &CommandMap, level: u16) {
        for (i, item) in sub.iter().enumerate() {
            let (k, v) = item;

            let is_root = level == 0;
            let is_last = sub.len() == i + 1;
            let is_exec = v.exec.is_some();

            // Create indentation
            for _ in 0..level {
                out.push_str(" ");
            }

            // Dont add branches for root elements
            if !is_root {
                out.push_str(if is_last { "└── " } else { "├── " });
            }

            // Exec commands should be **bold**
            out.push_str(&if is_exec {
                format!("**{}** `exec`", k)
            } else {
                format!("*{}*", k)
            });

            // Description if applicable
            if let Some(desc) = &v.description {
                out.push_str(&format!(" {}", desc));
            }

            // Print
            out.push_str("\n");

            // Print the subcommands
            if let Some(sub) = &v.subcommand {
                self.render_subcommands(out, sub, level + 1);
            }
        }
    }

    /// Print [ProfileDef] in a table
    pub fn print_profile(&self, name: &String, def: &ProfileDef) {
        // Render profile tree onto a String
        let mut out = String::new();
        self.render_subcommands(&mut out, &def.0, 0);

        // Use static markdown template
        let mdtemplate = TextTemplate::from(PROFILE_DEF_TEMPLATE_MD);
        let mut mdexpander = OwningTemplateExpander::new();

        mdexpander.set("profile", name);
        mdexpander.set_md("tree", out);

        self.skin.print_owning_expander(&mdexpander, &mdtemplate);
    }

    /// Prompt user for a profile
    pub fn select(&self, prompt: &str, opts: &Vec<String>) -> Result<String> {
        Ok(inquire::Select::new(prompt, opts.clone()).prompt()?)
    }

    /// Prompt user for text input
    pub fn input(&self, prompt: String) -> Result<String> {
        Ok(inquire::Text::new(&format!("{}:", prompt)).prompt()?)
    }

    /// Prompt the user for a number
    pub fn integer(&self, prompt: &str, min: i32, max: i32) -> Result<i32> {
        Ok(inquire::CustomType::<i32>::new(&format!(
            "{} (Between {} and {}):",
            prompt,
            if min == i32::MIN {
                "MIN".to_string()
            } else {
                min.to_string()
            },
            if max == i32::MAX {
                "MAX".to_string()
            } else {
                max.to_string()
            },
        ))
        .with_error_message(&format!("Please enter a valid integer between"))
        .with_validator(move |val: &i32| {
            Ok(if *val >= min && *val <= max {
                inquire::validator::Validation::Valid
            } else {
                inquire::validator::Validation::Invalid(inquire::validator::ErrorMessage::Custom(
                    format!("Value must be between {} and {}", min, max),
                ))
            })
        })
        .prompt()?)
    }

    /// Prompt for a y/n response
    pub fn confirm(&self, prompt: &str) -> Result<bool> {
        Ok(inquire::Confirm::new(prompt).with_default(true).prompt()?)
    }
}
