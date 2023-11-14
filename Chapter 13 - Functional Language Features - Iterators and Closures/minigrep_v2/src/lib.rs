use std::{env, error::Error, fs};

pub struct Config {
    needle: String,
    haystack: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // discard the first item (name of program being run)
        args.next();

        let needle = match args.next() {
            Some(v) => v,
            None => return Err("Didn't receive a search string."),
        };

        let haystack = match args.next() {
            Some(v) => v,
            None => return Err("Didn't receive a file path to search in."),
        };

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
    haystack
        .lines()
        .filter(|line| {
            (ignore_case && line.to_lowercase().contains(&needle.to_lowercase()))
                || line.contains(needle)
        })
        .collect()
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
