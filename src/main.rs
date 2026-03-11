use std::{collections::HashMap, env::Args, io::Read, str::SplitWhitespace, vec};
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

fn separator(command: &str) -> Vec<&str> {
    let vector_of_args: Vec<&str> = command.split_whitespace().collect();
    return vector_of_args;
}

fn commandParse(command_input: &str) -> Result<i32, String> {
    let mut inbuilt_commands = HashMap::<&str, fn(Vec<&str>)->Result<i32, String>>::new();
    inbuilt_commands.insert("exit", exit_program);
    inbuilt_commands.insert("echo", echo);

    // let mut parts = command_input.splitn(2, char::is_whitespace);

    let arguments = separator(command_input);
    let command_name = arguments.get(0);
    
    let function_pointer = match command_name {
        Some(command_name) => inbuilt_commands.get(command_name),
        None => return Err("No Command".to_string())
    };
    match function_pointer {
        Some(function_pointer) => return (function_pointer(arguments)),
        None => {println!("{}: command not found", command_name.unwrap()); return Err("Command not found".to_string())}
    };
    
}

fn exit_program(arg_array: Vec::<&str>) -> Result<i32, String> {
    Ok(-1)
}

fn echo(arg_array: Vec::<&str>) -> Result<i32, String> {
    // println!("{}", arg_array.len());
    for i in 1..arg_array.len() {
        if i > 1 {print!(" ");}
        print!("{}", arg_array[i]);
    }
    print!("\n");
    Ok(0)
}