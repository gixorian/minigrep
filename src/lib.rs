use std::{env, error::Error, fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn process_flags(&mut self, flags: &[&str]) -> Result<(), String> {
        for &flag in flags {
            if !flag.starts_with('-') || flag.len() == 1 {
                return Err(format!("Invalid flag syntax: {flag}"));
            }

            for ch in flag.chars().skip(1) {
                match ch {
                    'i' => self.ignore_case = true,
                    other => {
                        return Err(format!("Unknown flag: -{other}"));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn build(args: &[String]) -> Result<Config, &str> {
        let mut flags: Vec<&str> = Vec::new();
        let mut main_args = Vec::new();

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        for arg in args {
            if arg.starts_with("-") {
                if arg.len() > 1 {
                    flags.push(arg);
                }
            } else {
                main_args.push(arg);
            }
        }

        if main_args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = main_args[1].clone();
        let file_path = main_args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let mut cfg = Config {
            query,
            file_path,
            ignore_case,
        };

        if let Err(e) = cfg.process_flags(&flags) {
            eprintln!("Failed processing flags.\n{e}");
            process::exit(1);
        }

        Ok(cfg)
    }
}

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
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
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
