use anyhow::{Context, Result};
use std::fs;

mod cli;
pub use cli::Cli;
use cli::Format;

mod color_config;
mod target;

use color_config::ColorConfig;
use target::Alacritty;
use target::Target;
use target::VscodeIntegratedTerminal;
use target::WindowsTerminal;

pub fn run(args: Cli) -> Result<()> {
    let content = fs::read_to_string(&args.color_config_path).with_context(|| {
        format!(
            "could not read file `{}`",
            &args.color_config_path.display()
        )
    })?;

    let color_config: ColorConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse ColorConfig file `{}`", &content))?;

    let target: Box<dyn Target> = match &args.format {
        Format::VscodeIntegratedTerminal => Box::new(VscodeIntegratedTerminal::from(color_config)),
        Format::WindowsTerminal => Box::new(WindowsTerminal::from(color_config)),
        Format::Alacritty => Box::new(Alacritty::from(color_config)),
    };

    let stem = args.color_config_path.file_stem().unwrap();
    target.write_file(stem.to_str().unwrap())?;

    Ok(())
}
