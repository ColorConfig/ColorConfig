use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Solongo {
    metadata: Metadata,
    colors: Colors,
}
#[derive(Serialize, Deserialize)]
pub struct Colors {
    primary: PrimaryColors,
    normal: AnsiColors,
    bright: AnsiColors,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    version: String,
    author: String,
    website: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Serialize, Deserialize)]
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
    let content = fs::read_to_string(&filename)
       .expect("could not read file");

    let content: Solongo = toml::from_str(&content).expect("toml error");
    let serialized = serde_json::to_string_pretty(&content)
        .expect("failed to jsonify Solongo");

    let mut file = File::create(format!("{}.json", &filename)).expect("failed to create file");
    file.write_all(serialized.as_bytes()).expect("failed to write file");

    assert_eq!(content.colors.normal.black, "#13181b");
}
