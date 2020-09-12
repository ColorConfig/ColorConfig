use std::path::PathBuf;

use structopt::StructOpt;
use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum Format {
        WindowsTerminal,
        VscodeIntegratedTerminal,
        Alacritty,
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub color_config_path: PathBuf,
    #[structopt(long, possible_values = &Format::variants(), case_insensitive = true)]
    pub format: Format,
}
