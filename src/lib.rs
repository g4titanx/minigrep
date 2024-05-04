use std::env;
use std::error::Error;
use std::fs;

///pub struct Config {
///pub query: String,
///pub file_path: String,
///pub ignore_case: bool,
///}
///
///impl Config {
///    pub fn build(
///        mut args: impl Iterator<Item = String>
/// ) -> Result<Config, &'static str> {
///args.next();
/// 
///let query = match args.next() { //next() returns a Result type
///Some(arg) => arg,
///None => return Err("Didn't get a query string"),
///};
///
///let file_path = match args.next() {
///Some(arg) => arg,
///None => return Err("Didn't get a file path"),
/// };
///let ignore_case = env::var("IGNORE_CASE").is_ok();
/// Ok(Config {
/// query,
/// file_path,
/// ignore_case,
/// })
/// }
///}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() { //next() returns a Result type
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

//Box<dyn Error> was used cause fs::read_to_string(config.file_path) might fail for dyn(dynamic) reasons
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let result = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<_>>();

    if result.is_empty(){
            vec!["query not found"]
    } else {
            result
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    let result = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<_>>();

    if result.is_empty(){
        vec!["query not found"]
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Pick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["query not found"],
            search_case_insensitive(query, contents)
        );
    }
}