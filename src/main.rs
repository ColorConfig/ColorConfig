use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("no filename given");
    let content = fs::read_to_string(filename)
       .expect("could not read file");

    for line in content.lines() {
        println!("{}", line);
    }
}
