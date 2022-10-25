use std::error::Error;
use std::fs;

pub fn run(query: &str, file_path: &str, ignore_case: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let results = if ignore_case {
        search_case_insensitive(query, contents.as_str())
    } else {
        search_case_sensitive(query, contents.as_str())
    };

    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE_SENSITIVE_CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
    const CASE_INSENSITIVE_CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

    #[test]
    fn no_results_case_sensitive() {
        assert_eq!(
            Vec::<&str>::new(),
            search_case_sensitive("rUsT", CASE_SENSITIVE_CONTENTS)
        );
    }

    #[test]
    fn one_result_case_sensitive() {
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive("duct", CASE_SENSITIVE_CONTENTS)
        );
    }

    #[test]
    fn no_results_case_insensitive() {
        assert_eq!(
            Vec::<&str>::new(),
            search_case_insensitive("C++", CASE_INSENSITIVE_CONTENTS)
        );
    }

    #[test]
    fn one_result_case_insensitive() {
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive("rUsT", CASE_INSENSITIVE_CONTENTS)
        );
    }
}
