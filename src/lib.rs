use std::fs;
use std::io::Error;

/// Searches `contents` of file at `file_path` for lines that contain `query` and prints those lines.
/// Parameter `ignore_case` determines whether case is ignored while searching.
/// Calls either [`search_case_insensitive`] or [`search_case_sensitive`].
///
/// # Errors
///
/// This function will return an error when `fs::read_to_string(file_path)` returns an error.
/// See [`fs::read_to_string`].
pub fn run(query: &str, file_path: &str, ignore_case: bool) -> Result<(), Error> {
    let contents = fs::read_to_string(file_path)?;

    let results = if ignore_case {
        search_case_insensitive(query, &contents)
    } else {
        search_case_sensitive(query, &contents)
    };

    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

/// Filters lines from `contents` where `line.to_lowercase()` contains `query.to_lowercase()`.
///
/// # Examples
/// Returns two lines.
/// ```
/// use grep::search_case_insensitive;
/// let query = "Duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three
/// Duct tape.";
/// assert_eq!(
///     vec!["safe, fast, productive.", "Duct tape."],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Filters lines from `contents` where `line` contains `query`.
///
/// # Examples
///
/// Returns one line.
/// ```
/// use grep::search_case_sensitive;
/// let query = "Duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three
/// Duct tape.";
/// assert_eq!(
///     vec!["Duct tape."],
///     search_case_sensitive(query, contents)
/// );
/// ```
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_lines_case_insensitive() {
        let query = "Garbage collection";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(Vec::<&str>::new(), search_case_insensitive(query, contents));
    }

    #[test]
    fn zero_lines_case_sensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(Vec::<&str>::new(), search_case_sensitive(query, contents));
    }
}
