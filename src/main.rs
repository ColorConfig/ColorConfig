use std::env;

fn main() {
    let filename = env::args().nth(1).expect("no filename given");
    println!("filename: {}", filename);
}
