use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
RuSt:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["RuSt:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
