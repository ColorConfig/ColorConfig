use std::env;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]

pub struct Solongo {
    metadata: Metadata,
    colors: Colors,
}
#[derive(Deserialize)]
pub struct Colors {
    primary: PrimaryColors,
    normal: AnsiColors,
    bright: AnsiColors,
}

#[derive(Deserialize)]
struct Metadata {
    name: String,
    version: String,
    author: String,
    website: Option<String>,
}

#[derive(Deserialize)]
struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Deserialize)]
struct AnsiColors {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

fn main() {
    let filename = env::args().nth(1).expect("no filename given");
    let content = fs::read_to_string(filename)
       .expect("could not read file");

    let content: Solongo = toml::from_str(&content).expect("toml error");

    assert_eq!(content.colors.normal.black, "#13181b");
}
