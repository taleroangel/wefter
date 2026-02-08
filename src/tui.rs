use crate::{
    cli::Params,
    fs::{dirs::DirCfg, res::ResourceDirTable},
};
use anyhow::{Ok, Result};
use clap::CommandFactory;
use clap_help::Printer;
use termimad::{
    MadSkin,
    crossterm::style::Color,
    minimad::{OwningTemplateExpander, TextTemplate},
};

/// Template for the help description
const HELP_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/help.md"
));

/// Template for showing resources
const RESOURCE_LIST_TEMPLATE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/resource_list_template.md"
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

    /// Print help message using Tui skin
    pub fn print_help(&self) {
        Printer::new(Params::command())
            .with("introduction", HELP_TEMPLATE_MD)
            .with_skin(self.skin.clone())
            .print_help();
    }

    /// Pretty print a table with all available resources
    pub fn print_resources(&self, resources: &ResourceDirTable, dirs: &DirCfg) {
        // Use static markdown template
        let mdtemplate = TextTemplate::from(RESOURCE_LIST_TEMPLATE_MD);
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
                    "resource-autodetect",
                    if v.autodetect.is_some() {
                        "**✓**"
                    } else {
                        "*✗*"
                    },
                );
        }

        // Print table
        self.skin.print_owning_expander(&mdexpander, &mdtemplate);
    }

    /// Prompt user for a profile
    pub fn select_profile(&self, profiles: &Vec<String>) -> Result<String> {
        Ok(inquire::Select::new("Select a profile", profiles.clone()).prompt()?)
    }
}
