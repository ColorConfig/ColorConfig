use anyhow::{Context, Result};
use colorconfig::{Cli, ColorConfig, TargetRegstry};
use std::fs;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    if let Err(err) = run(args) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

pub fn run(args: Cli) -> Result<()> {
    let content = fs::read_to_string(&args.color_config_path).with_context(|| {
        format!(
            "could not read file `{}`",
            &args.color_config_path.display()
        )
    })?;

    let registry = TargetRegstry::with_bulitins();
    let color_config: ColorConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse ColorConfig file `{}`", &content))?;

    let from_config = registry
        .get_from_config(&args.format.to_string())
        .expect("tool must guarantee this unless we have plugin system");
    let target = from_config(color_config);
    let stem = args.color_config_path.file_stem().unwrap();
    target.write_to_target_path(stem.to_str().unwrap(), false)?;

    Ok(())
}
