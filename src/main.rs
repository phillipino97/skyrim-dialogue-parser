use std::{env, io::Write, process::exit};
//use colored::Colorize;
use peak_alloc::PeakAlloc;
use serde::Serialize;

mod strings;
mod parsers;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Failed to provide command line arguments, please use the -help flag for information on flags");
        exit(1);
    }

    let mut env_state = 0;
    let mut dir: String = "".to_string();
    let mut esx = "".to_string();
    let mut ilstrings = "".to_string();
    let mut strings = "".to_string();
    let mut output = "".to_string();
    let mut first = true;
    for arg in args {
        if first {
            first = false;
            continue
        }
        if arg == "-dir" {
            env_state = 1;
            continue
        } else if arg == "-esx" {
            env_state = 2;
            continue
        } else if arg == "-ilstrings" {
            env_state = 3;
            continue
        } else if arg == "-strings" {
            env_state = 4;
            continue
        } else if arg == "-out" {
            env_state = 5;
            continue
        } else if arg == "-help" || arg == "-h" {
            println!("Command Line Input:\n-dir [directory] (Directory path containing .esx, .ILSTRINGS, and .STRINGS files)");
            println!("-esx [esx file name] (Name of the .esx file that you are analyzing, you need to add the file name extension)");
            println!("-ilstrings [ILSTRINGS file name] (Name of the .ILSTRINGS file that you are using, you need to add the file name extension)");
            println!("-strings [STRINGS file name] (Name of the .STRINGS file that you are using, you need to add the file name extension)");
            println!("-out [output file name] (Name of the output file, this program outputs data in a json format so you must add .json as the file extension)");
            println!("\nNote: For directory paths that include whitespace, please encapsulate them in quotation marks\n     All flags must be set for the program to run");
            exit(0);
        }

        if env_state == 0 {
            println!("Flag syntax is not correct, use the -help flag for more information, exiting");
            exit(1);
        } else if env_state == 1 {
            if dir == "" {
                if arg.chars().next().unwrap() == '"' {
                    dir = (&arg[1..arg.len()]).parse().unwrap();
                    if arg.chars().nth(arg.len()-1).unwrap() == '"' {
                        dir = dir[0..dir.len() - 1].parse().unwrap()
                    }
                } else {
                    dir = arg;
                    env_state = 0;
                }
            } else {
                if arg.chars().nth(arg.len()-1).unwrap() == '"' {
                    dir += &*(" ".to_string() + &arg[0..arg.len() - 1]);
                    env_state = 0;
                } else {
                    dir += &*(" ".to_string() + &arg[0..arg.len()]);
                }
            }
        } else if env_state == 2 {
            esx = arg;
            env_state = 0;
            continue
        } else if env_state == 3 {
            ilstrings = arg;
            env_state = 0;
            continue
        } else if env_state == 4 {
            strings = arg;
            env_state = 0;
            continue
        } else if env_state == 5 {
            output = arg;
            env_state = 0;
            continue
        }
    }

    if dir.len() == 0 || esx.len() == 0 || ilstrings.len() == 0 || strings.len() == 0 || output.len() == 0 {
        println!("Not all flags set, use the -help flag for more information, exiting");
        exit(1);
    }

    if dir.chars().nth(dir.len()-1).unwrap() != '/' || dir.chars().nth(dir.len()-1).unwrap() != '\\' {
        if dir.contains("\\") {
            dir += "\\";
        } else if dir.contains("/") {
            dir += "/";
        }
    }

    let ilstring_data = strings::ilstring_lookup::get_ilstring_data(dir.clone() + &*ilstrings);
    if ilstring_data.as_ref().is_err() {
        exit(1);
    }

    let string_data = strings::string_lookup::get_string_data(dir.clone() + &*strings);
    if string_data.as_ref().is_err() {
        exit(1);
    }

    let finished_ilstring_data = ilstring_data.unwrap();
    let finished_string_data = string_data.unwrap();

    let esm_file = dir.clone() + &*esx;
    let esm_data = parsers::esm_parser::get_top_level_data(&esm_file, &finished_ilstring_data, &finished_string_data);
    let dial_data = parsers::esm_parser::get_dial_group_data(&esm_file, esm_data, &finished_ilstring_data, &finished_string_data);

    //let current_mem = PEAK_ALLOC.current_usage_as_mb();
	//println!("This program currently uses {} MB of RAM.\n\n", current_mem.to_string().red());

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    dial_data.serialize(&mut ser).unwrap();
    //println!("{}", String::from_utf8(buf).unwrap());
    let mut file = std::fs::File::create(dir + &*output).unwrap();
    let check = file.write_all(String::from_utf8(buf).unwrap().as_bytes());

    if let Err(_err) = check {
        exit(1);
    }

    exit(0)

    /*let mut s = String::new();

    println!("Before your search add the keyword for the data you are searching for");
    println!("For player dialogue enter 'FULL:<key>' and for dialogue enter 'NAM1:<key>'");

    loop {
        print!("Enter your directory search: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        if s.eq("exit") {
            break;
        } else if s.starts_with("FULL:") {
            let input_as_bytes = u32::try_from(s[5..].parse::<i32>().unwrap()).unwrap();

            match finished_string_data.get(&input_as_bytes) {
                Some(data) => {println!("\n{}\n", data.cyan())},
                None => println!("\n{}\n", "Not found!".red())
            }
        } else if s.starts_with("NAM1:") {
            let input_as_bytes = u32::try_from(s[5..].parse::<i32>().unwrap()).unwrap();

            match finished_ilstring_data.get(&input_as_bytes) {
                Some(data) => println!("\n{}\n", data.green()),
                None => println!("\n{}\n", "Not found!".red())
            }
        }
        s = "".to_string();
    }*/
}
