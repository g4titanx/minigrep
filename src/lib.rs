#[cfg(test)]

use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build (args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

//Box<dyn Error> was used cause fs::read_to_string(config.file_path) might fail for dyn(dynamic) reasons
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.file_path)?;
    Ok(())
}