use std::env;
mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Reading from {}", config.filename);   
    lib::readfile(config.filename);
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("not enough arguments");
        }
        let filename = args[1].clone();
        Config { filename }
    }
}
