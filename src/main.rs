use std::{clone, collections::{HashMap, HashSet}, env::{self, Args}, f32::consts::E, fs::{File, metadata}, hash::Hash, io::{Read, Stderr, StdoutLock, stderr, stdout}, os::unix::fs::PermissionsExt, path, process::{Command, Stdio}, str::SplitWhitespace, thread::AccessError, vec};
#[allow(unused_imports)]
use std::io::{self, Write};
// use std::process::Stdio;

impl From<&Output> for Stdio {
    fn from(s: &Output) -> Self {
        match s {
            Output::Stdout => Stdio::inherit(),
            Output::Stderr => Stdio::inherit(),
            Output::File(file) => Stdio::from(file.try_clone().unwrap()),
        }
    }
}
struct out_stream {
    stdout: Output,
    stderr: Output,
}

enum Output {
    Stdout,
    Stderr,
    File(std::fs::File)
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Stdout => stdout().write(buf),
            Output::Stderr => stderr().write(buf),
            Output::File(file) => file.write(buf)
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Stdout => stdout().flush(),
            Output::Stderr => stderr().flush(),
            Output::File(file) => file.flush(),
        }
    }

}
impl Clone for Output {
    fn clone(&self) -> Self {
        match self {
            Output::Stdout => Output::Stdout,
            Output::Stderr => Output::Stderr,
            Output::File(file) => Output::File(file.try_clone().unwrap())
        }
    }
}

use bytes::buf::Writer;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command_input = String::new();
        io::stdin().read_line(&mut command_input);
        command_input = command_input.trim().to_string();
        let command_return = command_parse(&command_input);
        match command_return {
            Ok(-1) => break,
            Ok(_) => continue,
            Err(_) => continue
        }
    }
}

fn separator(command: &str) -> Vec<String> {
    let mut vector_of_args: Vec<String> = Vec::<String>::new();
    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        Err(_) => "".to_string()
    };

    let mut active_single_quotes = false; 
    let mut active_double_quotes = false;
    let mut i = 0;
    while i < command.len() {
        let mut substring_to_be_added: String = "".to_string();
        let mut j: usize = i;
        while j < command.len() {
            let character_j = command.as_bytes()[j] as char;

            if character_j == '\'' {
                if !active_double_quotes {active_single_quotes = !active_single_quotes; j+=1; continue;}
            }
            if character_j == '\"' {
                if !active_single_quotes {active_double_quotes = !active_double_quotes; j+=1; continue;}
            }

            //CHARACTER IF_ELSE
            //Double quotes not implemented yet
            if active_single_quotes {
                substring_to_be_added.push(character_j);
            }
            else if active_double_quotes {
                // substring_to_be_added.push(character_j);
                if character_j == '\\' && j+1 < command.len() {
                    let character_j1 = command.as_bytes()[j+1] as char;
                    if character_j1 == '\"' || character_j1 == '\\' || character_j1 == '$' || character_j1 == '`' || character_j1 == '\n'  {
                        substring_to_be_added.push(character_j1);
                        j+=1;
                    }
                    else {substring_to_be_added.push(character_j)};
                } else {substring_to_be_added.push(character_j)};
            }
            else {
                if character_j == ' ' {break;}
                else if character_j == '~' {substring_to_be_added.push_str(&home_dir);}
                else if character_j == '\\' {
                    if j+1 < command.len() {
                        j+=1;
                        substring_to_be_added.push(command.as_bytes()[j] as char);
                    }
                }
                else {substring_to_be_added.push(character_j);}
            }
            j+=1;
        }
        if substring_to_be_added.len() > 0 {vector_of_args.push(substring_to_be_added);}
        i = j + 1;
    }
    return vector_of_args;
}

fn command_parse(command_input: &str) -> Result<i32, String> {
    let mut inbuilt_commands = HashMap::<String, fn(Vec<String>, &HashSet<String>, out_stream)->Result<i32, String>>::new();
    inbuilt_commands.insert("exit".to_string(), exit_program);
    inbuilt_commands.insert("echo".to_string(), echo);
    inbuilt_commands.insert("type".to_string(), type_function);
    inbuilt_commands.insert("pwd".to_string(), print_working_dir);
    inbuilt_commands.insert("cd".to_string(), change_working_directory);


    let command_map: HashSet<String> = inbuilt_commands.keys().cloned().collect();

    let mut arguments = separator(command_input);
    let command_name = arguments.get(0).cloned();
    let mut Stdout_stream = Output::Stdout;
    let mut Stderr_stream = Output::Stderr;
    
    let mut fInd = arguments.len();
    for i in 0..arguments.len() {
        if (arguments[i] == ">" || arguments[i] == "1>") && i < arguments.len()-1  {
            Stdout_stream = Output::File(File::options().write(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if arguments[i] == "2>" && i < arguments.len()-1 {
            Stderr_stream = Output::File(File::options().write(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if arguments[i] == ">>" || arguments[i] == "1>>" && i < arguments.len()-1 {
            Stdout_stream = Output::File(File::options().append(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
        else if arguments[i] == "2>>" && i < arguments.len()-1 {
            Stderr_stream = Output::File(File::options().write(true).append(true).create(true).open(arguments[i+1].clone()).unwrap());
            fInd = std::cmp::min(fInd, i);
        }
    }
    arguments.resize(fInd, "".to_string());
    let mut output_stream = out_stream{stdout: Stdout_stream.clone(), stderr: Stderr_stream.clone()};
    
    match command_name {
        Some(command_name) => {
            let function_pointer = inbuilt_commands.get(&command_name);
            if let Some(fc_ptr) = function_pointer {
                return fc_ptr(arguments, &command_map, output_stream);
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

fn exit_program(_arg_array: Vec::<String>, _command_set: &HashSet<String>, mut _output_stream: out_stream) -> Result<i32, String> {
    Ok(-1)
}

fn echo(arg_array: Vec::<String>, _command_set: &HashSet<String>, mut output_stream: out_stream) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        if i > 1 {print!(" ");}
        write!(output_stream.stdout, "{}", arg_array[i]);
    }
    write!(output_stream.stdout, "\n");
    Ok(0)
}

fn path_finder(executable_name: &str) -> Result<String, bool> {
    let val = match env::var("PATH") {
        Ok(path_dir) => path_dir,
        Err(_) => "".to_string() 
    };
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

fn print_working_dir(_arg_array: Vec::<String>, _command_set: &HashSet<String>, mut output_stream: out_stream) -> Result<i32, String> {
    writeln!(output_stream.stdout, "{}", env::current_dir().unwrap().display());
    Ok(1)
}

fn change_working_directory(arg_array: Vec::<String>, _command_set: &HashSet<String>, mut output_stream: out_stream) -> Result<i32, String> {
    let mut newdir = match arg_array.get(1) {
        Some(dir) => dir.clone(),
        None => match env::var("HOME") {
            Ok(home) => home,
            Err(_) => return Err("No directory".to_string())
        }
    };
    match env::set_current_dir(&newdir) {
        Ok(_) => return Ok(0),
        Err(_) => {writeln!(output_stream.stderr,"{}: No such file or directory", newdir); return Err("directory doesn't exist".to_string())}
    }

}

fn type_function(arg_array: Vec::<String>, command_set: &HashSet<String>, mut output_stream: out_stream) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        let command_to_search = arg_array.get(i);
        match command_to_search {
            Some(command_to_search) => {
                if command_set.contains(command_to_search) {
                    writeln!(output_stream.stdout, "{} is a shell builtin", command_to_search);
                    continue;
                }
                // Check if in PATH
                match path_finder(command_to_search) {
                    Ok(val) => {writeln!(output_stream.stdout, "{} is {}/{}", command_to_search, val, command_to_search); return Ok(0);},
                    Err(_) => {}
                };


                // println!("{}: not found", command_to_search);
                writeln!(output_stream.stderr, "{}: not found", command_to_search);
            },
            None => {}
        }
    }

    Ok(0)
}