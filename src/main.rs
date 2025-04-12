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
    let mut buffer: HashMap<String, Vec<String>> = HashMap::new();
    //buffer.insert(String::from("no name"), Vec::new());
    let mut current_buffer: String = String::from("no name");
    let mut data: String = String::new();
    let mut reinit: bool = false;
    let mut initstate: InitState = InitState::AddBuffer;
    let mut lua_mods: Vec<LuaInit> = Vec::new();
    let mut lua_init: bool = true;

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
                            lua_init = false;
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
                                lua_init = false;
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

    // NOTE: main code
    println!("Mini-editor (type 'h' for help)");
    println!("                   __        \n.--------.-----.--|  |══     \n|        |  -__|  _  |════   \n|__|__|__|_____|_____|═══════\n");
    plugin_init(lua_init, &mut lua_mods);
    loop {
        init(&mut buffer, &mut current_buffer, &mut data, &mut initstate);
        reinit = false;
        'editing: loop {
            parse(input(), &mut buffer, &mut reinit, &mut initstate, &mut current_buffer, &mut data);
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

fn parse(
    cmd: Vec<String>, 
    buffer: &mut HashMap<String, Vec<String>>,
    reinit: &mut bool, 
    initstate: &mut InitState, 
    current_buffer: &mut String, 
    data: &mut String) 
{
    println!("{:?}", cmd);
    if cmd.len() > 0 {
        if cmd[0] == "q" {
            std::process::exit(0);
        }
        if cmd[0] == "new" {
            *reinit = true;
            *initstate = InitState::AddBuffer;

        }
        if cmd[0] == "remove" {
            let mut name = String::new();
            for i in &cmd[1..] {
                if !name.is_empty() {
                    name.push(' ');
                }
                name.push_str(i);
            }
            *reinit = true;
            *initstate = InitState::RemoveBuffer;
            *data = name.clone();
        }
        if cmd[0] == "name" {
            for key in buffer.keys() {
                print!("|{}| ", key);
            }
            print!("\n")
        }
        if cmd[0] == "current" {
            println!("/{}/", current_buffer);
            
        }
    }
}

fn plugin_init(lua_init: bool, lua_mods: &mut Vec<LuaInit>) {
    if lua_init == true {
        ()
    }
}

fn init(
    buffer: &mut HashMap<String, Vec<String>>, 
    current_buffer: &mut String,
    data: &mut String,
    initstate: &mut InitState
    ) 
{
    match initstate {
        InitState::AddBuffer => {
            let mut index = String::new();
            if buffer.contains_key(&String::from("no name")) {
                let mut i = 1;
                while buffer.contains_key(&format!("no name {}", i)) {
                    i += 1;}
                index = format!(" {}", i);
            }
            buffer.insert(format!("{}{}", "no name", index), Vec::new());
            *current_buffer = format!("{}{}", "no name", index);
            *data = String::new();
            *initstate = InitState::Nothing;
        },
        InitState::RemoveBuffer => {
            print!("\tDo you want to remove the buffer?(y/n): ");
            let agreement = input()[0].to_lowercase();
            if ["y", "yes", "fuck"].contains(&&*agreement) {
                buffer.remove(data);
                if buffer.is_empty() {
                    std::process::exit(0);
                }
            }
        },
        InitState::ChangeBuffer => {
            if buffer.contains_key(current_buffer) && current_buffer.starts_with("a'(") { // TODO: 
                //buffer.remove()
                ()
            }
        },
        InitState::MakeAssociatedBuffer => { // TODO: 
            buffer.insert(format!("a'({}", current_buffer), Vec::new());

        },
        InitState::RenameBuffer => { // TODO:
            if let Some(value) = buffer.remove(current_buffer) {
                buffer.insert(data.clone(), value);
            }
        },
        InitState::Nothing => {
            ()
        }
    }
}

enum InitState { //init()
    AddBuffer,
    RemoveBuffer,
    ChangeBuffer,
    MakeAssociatedBuffer,
    RenameBuffer,
    Nothing
}

enum LuaInit { //lua_mods: Vec<LuaInit>
    Nothing,
    Print,
    Input,
    Command 
}
