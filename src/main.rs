use std::{collections::{HashMap, HashSet}, env::Args, hash::Hash, io::Read, str::SplitWhitespace, vec};
#[allow(unused_imports)]
use std::io::{self, Write};

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
    let vector_of_args: Vec<String> = command.split_whitespace().collect();
    return vector_of_args;
}

fn commandParse(command_input: &str) -> Result<i32, String> {
    let mut inbuilt_commands = HashMap::<String, fn(Vec<String>, &HashSet<String>)->Result<i32, String>>::new();
    inbuilt_commands.insert("exit".to_string(), exit_program);
    inbuilt_commands.insert("echo".to_string(), echo);
    inbuilt_commands.insert("type".to_string(), type_function);

    let command_map: HashSet<String> = inbuilt_commands.keys().cloned().collect();
    // let mut parts = command_input.splitn(2, char::is_whitespace);

    let arguments = separator(command_input);
    let command_name = arguments.get(0);
    
    let function_pointer = match command_name {
        Some(command_name) => inbuilt_commands.get(command_name),
        None => return Err("No Command".to_string())
    };
    match function_pointer {
        Some(function_pointer) => return (function_pointer(arguments, &command_map)),
        None => {println!("{}: command not found", command_name.unwrap()); return Err("Command not found".to_string())}
    };
    
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

fn type_function(arg_array: Vec::<String>, commandSet: &HashSet<String>) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        let command_to_search = arg_array.get(i);
        match command_to_search {
            Some(command_to_search) => {
                if commandSet.contains(command_to_search) {
                    println!("{} is a shell builtin", command_to_search);
                }



                println!("{}: not found", command_to_search);
            },
            None => {}
        }


        
    }

    Ok(0)
}