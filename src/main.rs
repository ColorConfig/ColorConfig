use structopt::StructOpt;

fn main() {
    let args = colorconfig::Cli::from_args();
    if let Err(err) = colorconfig::run(args) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
