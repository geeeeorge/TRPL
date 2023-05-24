use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f: File = File::open(config.filename)?;

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    println!("With test:\n{}", contents);
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Self { query, filename })
    }


}
