use std::{env, error::Error, fs};

pub struct Config {
    needle: String,
    haystack: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("minigrep expects to receive 2 arguments.");
        }

        let needle = args[1].clone();
        let haystack = args[2].clone();

        let ignore_case = env::var("CASEIN").is_ok();

        Ok(Config {
            needle,
            haystack,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for '{}' in: {} ...",
        config.needle, config.haystack
    );

    let contents = fs::read_to_string(config.haystack)?;
    let needle = config.needle;

    for (num, line) in search(&needle, &contents, config.ignore_case)
        .iter()
        .enumerate()
    {
        println!("{}: {}\n", num, line);
    }

    Ok(())
}

pub fn search<'a>(needle: &str, haystack: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        let mut test_line = String::from(line);
        let mut test_needle = String::from(needle);

        if ignore_case {
            test_line = test_line.to_lowercase();
            test_needle = test_needle.to_lowercase();
        }

        if test_line.contains(&test_needle) {
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
        let needle = "duct";

        let haystack = "\
Rust:
safe, fast, productive.
Duct tape.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(needle, haystack, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = "\n
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(needle, haystack, true));
    }
}
