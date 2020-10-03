use std::path::PathBuf;

use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum Format {
        ColorConfig,
        Alacritty,
        VscodeIntegratedTerminal,
        WindowsTerminal,
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub color_config_path: PathBuf,
    #[structopt(long, possible_values = &Format::variants(), case_insensitive = true)]
    pub format: Format,
}
