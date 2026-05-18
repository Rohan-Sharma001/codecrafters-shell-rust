use crate::parser::redirect::command_parse;
use crate::terminal::line_editor::terminal_read;
use std::io::{self, Write};

pub fn shell() {
    let mut command_input = String::new();
    loop {
        // print!("$ ");
        io::stdout().flush();
        terminal_read(&mut command_input);
        print!("\n");
        // println!("{}", command_input);
        // let mut command_input = String::new();
        command_input = command_input.trim().to_string();
        let command_return = command_parse(&command_input);
        io::stdout().flush();

        match command_return {
            Ok(-1) => break,
            Ok(_) => continue,
            Err(_) => continue
        }
    }
}
