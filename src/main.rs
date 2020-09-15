use std::fs;
use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;
use anyhow::{Context, Result};

mod color_config;
mod vscode_integrated_terminal;
mod windows_terminal;
mod alacritty;
mod cli;
mod filename;

use color_config::ColorConfig;
use vscode_integrated_terminal::VscodeIntegratedTerminal;
use windows_terminal::WindowsTerminal;
use alacritty::Alacritty;
use cli::Cli;
use cli::Format;
use filename::Filename;

fn main() -> Result<()> {
    let args = Cli::from_args();

    let content = fs::read_to_string(&args.color_config_path)
        .with_context(|| format!("could not read file `{}`", &args.color_config_path.display()))?;

    let filename = Filename::from(&args.color_config_path);

    let color_config: ColorConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse ColorConfig file `{}`", &content))?;

    match &args.format {
        Format::VscodeIntegratedTerminal => {
            let vscode = VscodeIntegratedTerminal::from(color_config);
            let vscode = serde_json::to_string_pretty(&vscode)?;
            let filename = format!("{}.{}.{}",
                filename.name,
                "vscode",
                "json"
            );
            write_file(filename, vscode)?;
        }
        Format::WindowsTerminal => {
            let winterm = WindowsTerminal::from(color_config);
            let winterm = serde_json::to_string_pretty(&winterm)?;
            let filename = format!("{}.{}.{}",
                filename.name,
                "windows",
                "json"
            );
            write_file(filename, winterm)?;
        }
        Format::Alacritty => {
            let alacritty = Alacritty::from(color_config);
            let alacritty = serde_yaml::to_string(&alacritty)?;
            let filename = format!("{}.{}.{}",
                filename.name,
                "alacritty",
                "yml"
            );
            write_file(filename, alacritty)?;
        }
    }

    Ok(())
}

fn write_file(filename: String, content: String) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
        .context("Failed to write file.")?;

    Ok(())
}
