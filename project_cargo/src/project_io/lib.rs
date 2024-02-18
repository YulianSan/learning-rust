use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Lack of parameters");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: &Config) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(&config.file_path)?;
    let matches_search = match config.ignore_case {
        true => search_case_insensitive(&config.query, &content),
        false => search(&config.query, &content),
    };

    println!("\nMatches:\n{}", matches_search.join("\n"));

    Ok(content)
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut matches_search = vec![];

    for line in content.lines() {
        if line.contains(&query) {
            matches_search.push(line);
        }
    }

    matches_search
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches_search = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            matches_search.push(line);
        }
    }

    matches_search
}

// run only in "cargo test"
#[cfg(test)]
mod tests {
    use super::*;
    // the \ tells to not put a new line
    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn return_one_line() {
        let query = "afe";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "pICK";

        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(&query, CONTENTS)
        );
    }
}
