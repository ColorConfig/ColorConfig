use anyhow::{Context, Result};
use std::fs;
use std::io::prelude::*;

mod cli;
pub use cli::Cli;
use cli::Format;

mod color_config;
mod formats;

use color_config::ColorConfig;
use formats::alacritty::Alacritty;
use formats::vscode_integrated_terminal::VscodeIntegratedTerminal;
use formats::windows_terminal::WindowsTerminal;

pub fn run(args: Cli) -> Result<()> {
    let content = fs::read_to_string(&args.color_config_path).with_context(|| {
        format!(
            "could not read file `{}`",
            &args.color_config_path.display()
        )
    })?;

    let stem = args
        .color_config_path
        .file_stem()
        .unwrap()
        // FIXME: this is anti-pattern. filename must be a Path.
        .to_str()
        .unwrap();
    let color_config: ColorConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse ColorConfig file `{}`", &content))?;

    match &args.format {
        Format::VscodeIntegratedTerminal => {
            let vscode = VscodeIntegratedTerminal::from(color_config);
            let vscode = serde_json::to_string_pretty(&vscode)?;
            let filename = format!("{}.{}.{}", &stem, "vscode", "json");
            write_file(filename, vscode)?;
        }
        Format::WindowsTerminal => {
            let winterm = WindowsTerminal::from(color_config);
            let winterm = serde_json::to_string_pretty(&winterm)?;
            let filename = format!("{}.{}.{}", &stem, "windows", "json");
            write_file(filename, winterm)?;
        }
        Format::Alacritty => {
            let alacritty = Alacritty::from(color_config);
            let alacritty = serde_yaml::to_string(&alacritty)?;
            let filename = format!("{}.{}.{}", &stem, "alacritty", "yml");
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
