use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Solongo {
    metadata: Metadata,
    colors: Colors,
}

impl Solongo {
    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(&self) {
            Ok(json) => {
                json
            },
            Err(_err) => {
                panic!("Failed to convert to JSON")
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Colors {
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
