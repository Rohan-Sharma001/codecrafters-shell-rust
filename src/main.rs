use std::{collections::HashMap, io::Read};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command_input = String::new();
        io::stdin().read_line(&mut command_input);
        commandParse(&command_input);
    }
}

fn commandParse(command_input: &str) {
    let mut inbuilt_commands = HashMap::<&str, fn()>::new();
    let command_name = command_input.split_whitespace().next();
    let function_pointer = match command_name {
        Some(command_name) => inbuilt_commands.get(command_name),
        None => return
    };
    match function_pointer {
        Some(function_pointer) => function_pointer(),
        None => println!("{}: command not found", command_name.unwrap())
    };
    io::stdout().flush().unwrap();
}
