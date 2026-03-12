use std::{collections::{HashMap, HashSet}, env::{self, Args}, fs::metadata, hash::Hash, io::Read, os::unix::fs::PermissionsExt, path, process::Command, str::SplitWhitespace, vec};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command_input = String::new();
        io::stdin().read_line(&mut command_input);
        command_input.trim();
        let command_return = commandParse(&command_input);
        match command_return {
            Ok(-1) => break,
            Ok(_) => continue,
            Err(_) => continue
        }
    }
}

fn separator(command: &str) -> Vec<String> {
    let vector_of_args: Vec<String> = command.split_whitespace().map(String::from).collect();
    return vector_of_args;
}

fn commandParse(command_input: &str) -> Result<i32, String> {
    let mut inbuilt_commands = HashMap::<String, fn(Vec<String>, &HashSet<String>)->Result<i32, String>>::new();
    inbuilt_commands.insert("exit".to_string(), exit_program);
    inbuilt_commands.insert("echo".to_string(), echo);
    inbuilt_commands.insert("type".to_string(), type_function);
    inbuilt_commands.insert("pwd".to_string(), print_working_dir);
    inbuilt_commands.insert("cd".to_string(), change_working_directory);


    let command_map: HashSet<String> = inbuilt_commands.keys().cloned().collect();
    // let mut parts = command_input.splitn(2, char::is_whitespace);

    let arguments = separator(command_input);
    let command_name = arguments.get(0);
    
    match command_name {
        Some(command_name) => {
            let function_pointer = inbuilt_commands.get(command_name);
            if let Some(fc_ptr) = function_pointer {
                return fc_ptr(arguments, &command_map);
            } else {
                let process_new = Command::new(command_name).args(arguments.iter().skip(1)).spawn();
                if let Ok(mut new_proc) = process_new {
                    new_proc.wait();
                } else {
                    println!("{}: not found", command_name);
                }
            }
        },
        None => return Err("No Command".to_string())
    };
    // match function_pointer {
    //     Some(function_pointer) => return (function_pointer(arguments, &command_map)),
    //     None => {println!("{}: command not found", command_name.unwrap()); return Err("Command not found".to_string())}
    // };
    Ok(0)
}

fn exit_program(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    Ok(-1)
}

fn echo(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        if i > 1 {print!(" ");}
        print!("{}", arg_array[i]);
    }
    print!("\n");
    Ok(0)
}

fn path_finder(executable_name: &str) -> Result<String, bool> {
    let val = env::var("PATH").unwrap();
    let mut vector_of_paths = val.split(':');
    let mut path_iterator = vector_of_paths.next();
    loop {
        match path_iterator {
            Some(directory) => {
                let file_path = format!("{}/{}", directory.to_string(),executable_name);
                let metadata_file = metadata(&file_path);
                match metadata_file {
                    Ok(val) => {
                        let perm = val.permissions().mode() & 0o111 != 0;
                        if perm {
                            return Ok(directory.to_string());
                        }
                    }
                    Err(_) => {}
                }
                
            },
            None => break
            
        }
        path_iterator = vector_of_paths.next();
    }
    return Err(false)
}

fn print_working_dir(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    println!("{}", env::current_dir().unwrap().display());
    Ok(1)
}

fn change_working_directory(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    let newdir = match arg_array.get(1) {
        Some(dir) => dir.clone(),
        None => match env::var("HOME") {
            Ok(home) => home,
            Err(_) => return Err("No directory".to_string())
        }
    };
    match env::set_current_dir(&newdir) {
        Ok(_) => return Ok(0),
        Err(_) => {println!("{}: No such file or directory", newdir); return Err("directory doesn't exist".to_string())}
    }

}

fn type_function(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        let command_to_search = arg_array.get(i);
        match command_to_search {
            Some(command_to_search) => {
                if commandSet.contains(command_to_search) {
                    println!("{} is a shell builtin", command_to_search);
                    continue;
                }
                // Check if in PATH
                match path_finder(command_to_search) {
                    Ok(val) => {println!("{} is {}/{}", command_to_search, val, command_to_search); return Ok(0);},
                    Err(_) => {}
                };


                println!("{}: not found", command_to_search);
            },
            None => {}
        }
    }

    Ok(0)
}