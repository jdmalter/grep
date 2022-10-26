use grep::run;
use std::io::ErrorKind;
use std::{env, process};

fn main() {
    let mut args = env::args().into_iter();
    args.next(); // Ignoring executable path

    let query = args.next().unwrap_or_else(|| {
        eprintln!(
            "User error: {{ Expected query and file path arguments respectively. \
            Found no arguments. }}"
        );
        process::exit(1)
    });

    let file_path = args.next().unwrap_or_else(|| {
        eprintln!(
            "User error: {{ Expected query and file path arguments respectively. \
            Found only query: {:?} }}",
            query
        );
        process::exit(1)
    });

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    if let Err(error) = run(&query, &file_path, ignore_case) {
        match error.kind() {
            ErrorKind::NotFound => eprintln!(
                "User error: {{ Expected any file at file path. \
                Found no file at file path: {:?} }}",
                file_path
            ),
            ErrorKind::PermissionDenied => eprintln!(
                "User error: {{ Expected an accessible file. \
                Permission denied at file path {:?} }}",
                file_path
            ),
            _ => eprintln!("Application error: {:?}", error),
        }
        process::exit(1)
    }
}
