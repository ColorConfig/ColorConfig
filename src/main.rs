use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod solongo;
mod vscode_integrated_terminal;

use solongo::Solongo;
use vscode_integrated_terminal::VscodeIntegratedTerminal;

fn main() {
    let filename = env::args().nth(1)
        .expect("no filename given");
    let content = fs::read_to_string(&filename)
        .expect("could not read file");
    let content: Solongo = toml::from_str(&content)
        .expect("toml error");
    let mut file = File::create(format!("{}.json", &filename))
        .expect("failed to create file");

    let vscode = VscodeIntegratedTerminal::from(content);
    let vscode = serde_json::to_string_pretty(&vscode).unwrap();

    file.write_all(vscode.as_bytes()).expect("failed to write file");
}
