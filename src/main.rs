use anyhow::Result;
use colorconfig::{target::Source, Cli, ColorConfig, TargetRegistry};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    if let Err(err) = run(args) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

pub fn run(args: Cli) -> Result<()> {
    let registry = TargetRegistry::with_bulitins();
    let color_config = ColorConfig::from_path(&args.color_config_path)?;
    let from_config = registry
        .get_from_config(&args.format.to_string())
        .expect("tool must guarantee this unless we have plugin system");
    let target = from_config(color_config);
    let color_name = args
        .color_config_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    target.write_to_target_path(color_name, false)?;

    Ok(())
}
