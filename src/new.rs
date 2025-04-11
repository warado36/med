use std::env;

const HELP_MESSAGE: &str = r#"Usage: program [OPTIONS]

Options:
  -m, --minimal   run with default configuration
  -h, --help      show this help message and exit
  -v, --version   show program's version and exit"#;

const VERSION: &str = "1.0-rust version";

fn main() {
    let mut args: Vec<String> = Vec::new();
    for arg in env::args().skip(1) {
        args.push(arg);
    }
    println!("{:?} {}", args, args.len());
    for arg in args {
        match arg.as_str() {
            s if s.starts_with("--") => {
                match s {
                    "--help" => {
                        println!("{}", HELP_MESSAGE);
                        std::process::exit(0);
                    },
                    "--version" => {
                        println!("{}", VERSION);
                        std::process::exit(0);
                    },
                    "-minimal" => {
                        ()
                    },
                    _ => {
                        println!("Error: Unknown argument");
                        std::process::exit(1);
                    }
                }

            },
            s if s.starts_with('-') => {
                for a in s.chars() {
                    match a {
                        'h' => {
                            println!("{}", HELP_MESSAGE);
                            std::process::exit(0);
                        },
                        'v' => {
                            println!("{}", VERSION);
                            std::process::exit(0);
                        },
                        'm' => {
                            ()
                        },
                        '-' => {
                            ()
                        },
                        _ => {
                            println!("Error: Unknown argument");
                            std::process::exit(1);
                        }
                    }
                }
            },
            _ => (),
        }
    }
}

