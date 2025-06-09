use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    #[arg(required_unless_present = "version")]
    pub pattern: Option<String>,

    /// The path to the file to read
    #[arg(required_unless_present = "version")]
    pub path: Option<PathBuf>,

    /// Ignore case sensitivity in the data
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Display current version information and exit
    #[arg(short = 'v', long = "version")]
    pub version: bool,
}

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    if args.version {
        version();
        return Ok(());
    }

    let contents = fs::read_to_string(args.path.unwrap())?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.pattern.unwrap(), &contents)
    } else {
        search(&args.pattern.unwrap(), &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn version() {
    print!(env!("CARGO_PKG_NAME"));
    println!("-v{}", env!("CARGO_PKG_VERSION"));
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
