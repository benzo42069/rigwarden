use std::{env, fs, path::Path, process::ExitCode};

use topology_devtools::fixture::validate_yaml;

const USAGE: &str = "usage: topology-devtools validate-fixture <path>";

fn main() -> ExitCode {
    let mut arguments = env::args_os();
    let _program = arguments.next();

    let Some(command) = arguments.next() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };

    if command != "validate-fixture" {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }

    let Some(path) = arguments.next() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };

    if arguments.next().is_some() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }

    let path = Path::new(&path);
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("{}: io_error: {error}", path.display());
            return ExitCode::from(1);
        }
    };

    match validate_yaml(&input) {
        Ok(()) => {
            println!("valid");
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for error in errors {
                eprintln!(
                    "{}: {} [{}] {}",
                    path.display(),
                    error.path,
                    error.code,
                    error.message
                );
            }
            ExitCode::from(1)
        }
    }
}
