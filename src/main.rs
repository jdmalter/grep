use grep::run;
use std::{env, process};

fn main() {
    let mut args = env::args().into_iter();
    args.next(); // Ignoring executable path

    let query = args.next().unwrap_or_else(|| {
        eprintln!("User error: {{ Expected query and file path. Found neither. }}");
        process::exit(1)
    });

    let file_path = args.next().unwrap_or_else(|| {
        eprintln!(
            "User error: {{ Expected query and file path. Found only query: {:?} }}",
            query
        );
        process::exit(1)
    });

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    if let Err(e) = run(query.as_str(), file_path.as_str(), ignore_case) {
        eprintln!("Application error: {:?}", e);
        process::exit(1)
    }
}
