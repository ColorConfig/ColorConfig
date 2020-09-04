use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod solongo;
use solongo::Solongo;

fn main() {
    let filename = env::args().nth(1).expect("no filename given");
    let content = fs::read_to_string(&filename)
       .expect("could not read file");

    let content: Solongo = toml::from_str(&content).expect("toml error");
    let serialized = serde_json::to_string_pretty(&content)
        .expect("failed to jsonify Solongo");

    let mut file = File::create(format!("{}.json", &filename)).expect("failed to create file");
    file.write_all(serialized.as_bytes()).expect("failed to write file");
}
