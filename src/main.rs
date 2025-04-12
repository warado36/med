use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;

const HELP_MESSAGE: &str = r#"Usage: program [OPTIONS] [PATH]

Options:
  -m, --minimal   run with default configuration (not working)
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
    let mut buffer: HashMap<&str, Vec<String>> = HashMap::new();
    let mut current_buffer: &str = "empty";
    let mut reinit: bool = false;
    let mut initstate: InitState = InitState::EmptyBuffer;

    // NOTE: parsing args
    {
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
                        "--minimal" => {
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

    // NOTE: init()
    println!("Mini-editor (type 'h' for help)");
    println!("                   __        \n.--------.-----.--|  |══     \n|        |  -__|  _  |════   \n|__|__|__|_____|_____|═══════\n");
    loop {
        init();
        reinit = false;
        'editing: loop {
            parse(input(), &mut reinit, &mut initstate);
            if reinit == true {
                println!("break and reinit");
                break 'editing
            }
        }
    }
}

//fn help() {
//
//}


fn input() -> Vec<String> {
    let mut cmd = String::new();
    print!("* ");
    io::stdout().flush().expect("Error: failed to flush stdout");
    io::stdin().read_line(&mut cmd).expect("Error: failed to read");
    let parts: Vec<String> = cmd.trim().split_whitespace().map(|s| s.to_string()).collect();
    parts
}

fn parse(cmd: Vec<String>, reinit: &mut bool, initstate: &mut InitState ) {
    println!("{:?}", cmd);
    if cmd.len() > 0 {
        if cmd[0] == "q" {
            std::process::exit(0);
        }
        if cmd[0] == "reinit" {
            *reinit = true;
            *initstate = InitState::ChangeBuffer;
        }
    }
}

fn init() {
    ()
}

enum InitState {
    AddBuffer,
    RemoveBuffer,
    ChangeBuffer,
    AssociateBuffer,
    EmptyBuffer
}
