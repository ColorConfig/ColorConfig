extern crate serde_with;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Solongo {
    metadata: Metadata,
    pub colors: Colors,
}

#[derive(Serialize, Deserialize)]
pub struct Colors {
    pub primary: PrimaryColors,
    pub normal: AnsiColors,
    pub bright: AnsiColors,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    version: String,
    author: String,
    website: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PrimaryColors {
    pub background: String,
    pub foreground: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}
