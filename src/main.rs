//use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;

const HELP_MESSAGE: &str = r#"Usage: program [OPTIONS] [PATH]

Options:
  -m, --minimal   run with default configuration
  -h, --help      show this help message and exit
  -v, --version   show program's version and exit"#;

const VERSION: &str = "1.0-rust version";

const HELP: &str = r#"Commands:
  ld           - list directories in the current path
  cd [path]    - change the current working directory
  a            - append text
  i[bfre]      - insert text
  p[:from:,to] - print buffer
  s [expr]     - search in buffer
  d[from:,to]  - delete from buffer
  w [file]     - write to file
  r [file]     - read from file
  q            - quit
  h            - help"#;


fn main() {
    // NOTE: initial memory allocation
//    let mut buffer: HashMap<&str, Vec<String>> = HashMap::new();
//    let mut current_buffer: &str = "empty";

    // NOTE: parsing args
    let mut args: Vec<String> = Vec::new();
    for arg in env::args().skip(1) {
        args.push(arg);
    }
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

    // NOTE: init()
    println!("Mini-editor (type 'h' for help)");
    println!("                   __        \n.--------.-----.--|  |══     \n|        |  -__|  _  |════   \n|__|__|__|_____|_____|═══════\n");
    loop {
        init();
        loop {
            let mut input_cmd = String::new();
            parse(input(&mut input_cmd));
        }
    }
}

//fn help() {
//
//}


fn input(input_cmd: &mut String) -> Vec<&str> {
    print!("* ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(input_cmd).expect("Error: failed to read");
    let parts: Vec<&str> = input_cmd.trim().split_whitespace().collect();
    parts
}

fn parse(cmd: Vec<&str>) {
    if cmd.len() > 0 {
        if cmd[0] == "q" {
            std::process::exit(0);
        }
    }
}

fn init() {
    ()
}

//enum InitState {
//    AddBuffer,
//    RemoveBuffer,
//    ChangeBuffer,
//    AssociateBuffer,
//    EmptyBuffer
//}
