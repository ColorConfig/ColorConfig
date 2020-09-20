use std::fs;
use std::io::prelude::*;

use anyhow::{Context, Result};

mod cli;
pub use cli::Cli;
use cli::Format;

mod color_config;
mod formats;
mod filename;

use color_config::ColorConfig;
use formats::vscode_integrated_terminal::VscodeIntegratedTerminal;
use formats::windows_terminal::WindowsTerminal;
use formats::alacritty::Alacritty;
use filename::Filename;

pub fn run(args: Cli) -> Result<()>{
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
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())
        .context("Failed to write file.")?;
    Ok(())
}
