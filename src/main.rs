use std::{collections::HashMap, io::Read};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command_input = String::new();
        io::stdin().read_line(&mut command_input);
        let command_return = commandParse(&command_input);
        match command_return {
            Ok(-1) => break,
            Ok(_) => continue,
            Err(_) => continue
        }
    }
}

fn commandParse(command_input: &str) -> Result<i32, String> {
    let mut inbuilt_commands = HashMap::<&str, fn()->Result<i32, String>>::new();
    inbuilt_commands.insert("exit", exit_program);
    let command_name = command_input.split_whitespace().next();
    let function_pointer = match command_name {
        Some(command_name) => inbuilt_commands.get(command_name),
        None => return Err("No Command".to_string())
    };
    match function_pointer {
        Some(function_pointer) => return (function_pointer()),
        None => {println!("{}: command not found", command_name.unwrap()); return Err("Command not found".to_string())}
    };
    // io::stdout().flush().unwrap();
}

fn exit_program() -> Result<i32, String> {
    Ok(-1)
}