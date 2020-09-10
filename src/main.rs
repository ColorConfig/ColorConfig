use std::fs;
use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;

mod solongo;
mod vscode_integrated_terminal;
mod windows_terminal;
mod alacritty;
mod cli;

use solongo::Solongo;
use vscode_integrated_terminal::VscodeIntegratedTerminal;
use windows_terminal::WindowsTerminal;
use alacritty::Alacritty;
use cli::Cli;
use cli::Format;

fn main() {
    let args = Cli::from_args();

    let filename = &args.solongo_path
        .into_os_string()
        .into_string()
        .unwrap();

    let format = &args.format;

    let content = fs::read_to_string(filename)
        .expect("could not read file");

    let solongo: Solongo = toml::from_str(&content)
        .expect("toml error");

    match format {
        Format::VscodeIntegratedTerminal => {
            let vscode = VscodeIntegratedTerminal::from(solongo);
            let vscode = serde_json::to_string_pretty(&vscode)
                .unwrap();
            write_file(&filename, vscode);
        }
        Format::WindowsTerminal => {
            let winterm = WindowsTerminal::from(solongo);
            let winterm = serde_json::to_string_pretty(&winterm)
                .unwrap();
            write_file(&filename, winterm);
        }
        Format::Alacritty => {
            let alacritty = Alacritty::from(solongo);
            let alacritty = serde_yaml::to_string(&alacritty)
                .unwrap();
            write_file(&filename, alacritty);
        }
    }
}

fn write_file(filename: &String, content: String) {
    let mut file = File::create(format!("{}.json", filename))
        .expect("failed to create file");
    file.write_all(content.as_bytes())
        .expect("failed to write file");
 }
