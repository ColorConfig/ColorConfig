use std::fs;
use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;

mod solongo;
mod vscode_integrated_terminal;
mod windows_terminal;
mod cli;

use solongo::Solongo;
use vscode_integrated_terminal::VscodeIntegratedTerminal;
use windows_terminal::WindowsTerminal;
use cli::Cli;

fn main() {
    let args = Cli::from_args();

    let filename = &args.solongo_path
        .into_os_string()
        .into_string()
        .unwrap();

    let content = fs::read_to_string(filename)
        .expect("could not read file");

    let content: Solongo = toml::from_str(&content)
        .expect("toml error");

    let winterm = WindowsTerminal::from(content);
    let winterm = serde_json::to_string_pretty(&winterm)
        .unwrap();

    write_file(&filename, winterm);
}

fn write_file(filename: &String, content: String) {
    let mut file = File::create(format!("{}.json", filename))
        .expect("failed to create file");
    file.write_all(content.as_bytes())
        .expect("failed to write file");
 }
