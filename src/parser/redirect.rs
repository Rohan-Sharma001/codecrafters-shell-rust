use crate::built_ins::inbuilt_commands;
use crate::exec::output::{out_stream, Output};
use crate::parser::tokenize::separator;
use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

pub fn command_parse(command_input: &String) -> Result<i32, String> { //COMMAND PARSER
    let mut arguments = separator(command_input);
    let command_name = arguments.get(0).cloned();
    let mut Stdout_stream = Output::Stdout;
    let mut Stderr_stream = Output::Stderr;
    
    let mut fInd = arguments.len();
    for i in 0..arguments.len() {
        if (arguments[i] == ">" || arguments[i] == "1>") && i < arguments.len()-1  {
            Stdout_stream = Output::File(File::options().truncate(true).write(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if arguments[i] == "2>" && i < arguments.len()-1 {
            Stderr_stream = Output::File(File::options().truncate(true).write(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if (arguments[i] == ">>" || arguments[i] == "1>>") && i < arguments.len()-1 {
            Stdout_stream = Output::File(File::options().append(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if arguments[i] == "2>>" && i < arguments.len()-1 {
            Stderr_stream = Output::File(File::options().append(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
    }
    arguments.resize(fInd, "".to_string());
    let mut output_stream = out_stream{stdout: Stdout_stream.clone(), stderr: Stderr_stream.clone()};
    
    match command_name {
        Some(command_name) => {
            let function_pointer = inbuilt_commands.get(command_name.as_str());
            if let Some(fc_ptr) = function_pointer {
                return fc_ptr(arguments, output_stream);
            } else {
                // let stdo = match  File::create(Stdout_file){
                //     Ok(out_file) => std::process::Stdio::from(out_file),
                //     Err(_) => Stdio::inherit()
                // };
                // let stde = match  File::create(Stderr_file){
                //     Ok(err_file) => std::process::Stdio::from(err_file),
                //     Err(_) => Stdio::inherit()
                // };
                let process_new = Command::new(&command_name).args(arguments.iter().skip(1)).stdout(Stdio::from((&Stdout_stream))).stderr(Stdio::from(&Stderr_stream)).spawn();
                if let Ok(mut new_proc) = process_new {
                    new_proc.wait();
                } else {
                    writeln!(output_stream.stderr, "{}: not found", command_name);
                }
            }
        },
        None => return Err("No Command".to_string())
    };
    Ok(0)
}
