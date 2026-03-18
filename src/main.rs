use std::{collections::{HashMap, HashSet}, env::{self}, ffi::c_long, fs::{self, File, metadata}, io::{Read, stderr, stdout}, os::unix::fs::PermissionsExt, process::{Command, Stdio}};
#[allow(unused_imports)]
use std::sync::LazyLock;
use std::io::{self, Write};
use termios::*;
type BuiltinHandler = fn(Vec<String>, out_stream) -> Result<i32, String>;
use std::collections::BTreeSet;
static inbuilt_commands: LazyLock<HashMap<&'static str, BuiltinHandler>> = LazyLock::new(|| {
    HashMap::from([
        ("exit", exit_program as BuiltinHandler),
        ("echo", echo as BuiltinHandler),
        ("type", type_function as BuiltinHandler),
        ("pwd", print_working_dir as BuiltinHandler),
        ("cd", change_working_directory as BuiltinHandler),
    ])
});

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
struct RawMode {
    org: Termios,
}
impl RawMode {
    fn new() -> Self {
        let fd = 0;
        let mut term = Termios::from_fd(fd).unwrap();
        let org = term.clone();
        term.c_lflag &= !(ICANON | ECHO);
        tcsetattr(fd, TCSANOW, &term).unwrap();
        RawMode {org}
    }
}
impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = tcsetattr(0, TCSANOW, &self.org).unwrap();
    }
}

fn main() {
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
        match command_return {
            Ok(-1) => break,
            Ok(_) => continue,
            Err(_) => continue
        }
    }
}

fn terminal_read(buffer: &mut String) {
    print!("$ ");
    io::stdout().flush();
    buffer.clear();
    let mut cursor = 0;
    let raw_term = RawMode::new();
    let mut char_in = [0u8; 1];
    // let mut buffer = String::new();
    let mut multi_output = false;

    loop {
        io::stdin().read_exact(&mut char_in).unwrap();
        let byte_read = char_in[0] as char;
        match char_in[0] {
            b'\n' => {break;}
            127 => {
                if cursor > 0 {
                    buffer.remove(cursor-1);
                    cursor-=1;
                }
                multi_output = false;
            }
            b'\t' => {
                match_command(buffer, &mut multi_output);
                // buffer.push(' ');
                cursor = buffer.len();
            }
            27 => {
                let mut buff = [0u8; 64];
                let mut char_inn = [0u8; 1];
                for i in 0..64 {
                    io::stdin().read_exact(&mut char_inn).unwrap();
                    buff[i] = char_inn[0];
                    // if buff[i].is_ascii_alphabetic() {break;}
                    if buff[i] <= 126 && buff[i] >= 64 && buff[i] != 91 {break;}
                }
                // io::stdin().read_exact(&mut buff).unwrap();
                match buff {
                    [91,68, ..] if cursor > 0 => cursor-=1,
                    [91,67, ..] if cursor < buffer.len() => cursor+=1,
                    [91,51,126, ..] if cursor < buffer.len()-1 => {buffer.remove(cursor);}
                    _ => {}
                }
                multi_output = false;
            }
            char => {
                buffer.insert(cursor, char as char);
                cursor+=1;
                multi_output = false;
            }
        }
        buffer.push(' ');
        print!("\r$ {}", buffer); //Send cursor to beginning of line -> write line
        print!("\x1b[K"); //Clear characters after buffer
        print!("\x1b[{}D", buffer.len()-cursor);
        buffer.pop();
        if buffer.ends_with('\x07') {buffer.pop();}
        io::stdout().flush();
    }
}

fn command_matches(prefix: &str) -> BTreeSet<String> {
    let mut VecSt = BTreeSet::<String>::new();
    for cmd in inbuilt_commands.keys() {
        if cmd.starts_with(prefix) {
            VecSt.insert(cmd.to_string());
        }
    }
    let path_dir_list = match env::var("PATH") {
        Ok(path_dir_list) => path_dir_list,
        Err(_) => "".to_string() 
    };
    let mut vector_of_paths = path_dir_list.split(':');
    let mut path_iterator = vector_of_paths.next();
    loop {
        match path_iterator {
                Some(path_dir) => {
                    match fs::read_dir(path_dir) {
                        Ok(rdir) => {
                            for entry in rdir {
                                if let Ok(valid_entry) = entry {
                                    let fname = valid_entry.file_name().into_string();
                                    if let Ok(fname_str) = fname && fname_str.starts_with(prefix) {
                                        VecSt.insert(fname_str);
                                    } 
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }            
                None => break
        }
        path_iterator = vector_of_paths.next();
    }
    VecSt
}

// fn longest_common_prefix(cmd_buffer: &mut String) {
//     let matches = command_matches(&cmd_buffer);
//     if matches.len() == 0 {
//         cmd_buffer.push('\x07');
//         io::stdout().flush();
//         return;
//     }
//     let mut longest_common_index = 0;
//     'counter: loop {
//         for cmd in &matches {
//             if let Some(char) = cmd.as_bytes().get(longest_common_index) {
//                 if *char != matches[0].as_bytes()[longest_common_index] {break 'counter;}
//             } else {
//                 break 'counter;
//             }
//         }
//         longest_common_index += 1;
//     }
//     let longest_common_str = String::from_utf8(matches[0].as_bytes()[0..longest_common_index].to_vec()).unwrap();
//     *cmd_buffer = longest_common_str;
//     if (longest_common_index == matches[0].as_bytes().len()) {cmd_buffer.push(' ');}
// }
// fn replace_current_token(buffer: &mut String, cursor: &mut usize, replacement: &str) {
//     buffer = 
// }
fn match_command(cmd_buffer: &mut String, multi_output: &mut bool) {
    let matches = command_matches(&cmd_buffer);
    if matches.len() == 0 {
        cmd_buffer.push('\x07');
        io::stdout().flush();
        return;
    }
    else if matches.len() == 1 {
        *cmd_buffer = matches.iter().next().unwrap().clone();
        cmd_buffer.push(' ');
    } 
    else if *multi_output {
        print!("\n");
        for element in matches {
            print!("{}  ", element);
        }
        print!("\n");
        *multi_output = false;
    }
    else {
        cmd_buffer.push('\x07');
        *multi_output = true;
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

fn exit_program(_arg_array: Vec::<String>, mut _output_stream: out_stream) -> Result<i32, String> {
    Ok(-1)
}

fn echo(arg_array: Vec::<String>, mut output_stream: out_stream) -> Result<i32, String> {
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

fn print_working_dir(_arg_array: Vec::<String>, mut output_stream: out_stream) -> Result<i32, String> {
    writeln!(output_stream.stdout, "{}", env::current_dir().unwrap().display());
    Ok(1)
}

fn change_working_directory(arg_array: Vec::<String>, mut output_stream: out_stream) -> Result<i32, String> {
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

fn type_function(arg_array: Vec::<String>, mut output_stream: out_stream) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        let command_to_search = arg_array.get(i);
        match command_to_search {
            Some(command_to_search) => {
                if inbuilt_commands.contains_key(command_to_search.as_str()) {
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
