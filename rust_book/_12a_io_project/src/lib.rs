use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    for line in search_n(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_n<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results_n = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.contains(&query) {
            results_n.push(line.trim());
        }
    }

    results_n
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)
    -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results_n = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results_n.push(line.trim());
        }
    }

    results_n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_n(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}