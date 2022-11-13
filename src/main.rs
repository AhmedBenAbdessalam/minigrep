use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        eprintln!("{err}");
        process::exit(1);
    }
}
