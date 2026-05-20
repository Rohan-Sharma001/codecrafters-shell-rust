use crate::built_ins::inbuilt_commands;
use std::{
    collections::BTreeSet,
    env,
    fs,
    io::{self, Write},
};
fn match_command(cmd_buffer: &mut String, multi_output: &mut bool, replace_cmd: bool) {
    let matches = match replace_cmd {
        true => command_matches(&cmd_buffer),
        false => file_matches(&cmd_buffer)
    };
    if matches.len() == 0 {
        // cmd_buffer.push('\x07');
        print!("\x07");
        io::stdout().flush();
        return;
    }
    else if matches.len() == 1 {
        *cmd_buffer = matches.iter().next().unwrap().clone();
        // cmd_buffer.push(' ');
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
        print!("\x07");
        *multi_output = true;
        match longest_common_prefix(matches) {
            Some(word) => {*cmd_buffer = word;return;}
            None => {}
        }
        // cmd_buffer.push('\x07');
        
    }

}
fn file_matches(token: &str) -> BTreeSet<String> { //Find matching files
    let mut VecSt = BTreeSet::<String>::new();
    let mut dir = env::current_dir().unwrap();
    let token_separate: Vec<&str> = token.split('/').collect();
    for i in 0..token_separate.len()-1 {
        dir.push(token_separate[i]);
    }
    let mut last_slash = (token.len()-1) as i32;
    while (last_slash >= 0 && token.as_bytes()[last_slash as usize] != b'/' ) {
        last_slash -= 1;
    }
    let to_autocomplete = token_separate[token_separate.len()-1];
    let read_dir = fs::read_dir(&dir).unwrap();
        for entry in read_dir {
            if let Ok(ent) = entry {
                let fpart = match last_slash {
                    ..0 => "",
                    _ => &token[0..(last_slash)as usize+1]
                };
                if (ent.file_name().to_string_lossy().starts_with(to_autocomplete)) {
                    // VecSt.insert(format!{"{}{}", fpart, ent.file_name().display()});
                    // VecSt.insert(format!("{:?}", ent.path()));
                    if (ent.path().is_dir()) {VecSt.insert(format!{"{}{}/", fpart, ent.file_name().display()});}
                    else {VecSt.insert(format!{"{}{} ", fpart, ent.file_name().display()});}
                }
            } else {
                break;
            }
        }
    VecSt
}
fn longest_common_prefix(matches: BTreeSet<String>) -> Option<String> {
    // let matches = command_matches(cmd_buffer);
    if matches.len() == 0 {
        //cmd_buffer.push('\x07');
        print!("\x07");
        io::stdout().flush();
        return None;
    }
    let ptr = matches.first().unwrap();
    for i in 0..ptr.len() {
        let mut flag_ = true;
        for m_ in matches.iter() {
            if i > m_.len()-1 || ptr.as_bytes()[i] != m_.as_bytes()[i] {flag_ = false;}
        }
        if flag_ == false {
            if i == 0 {return None;}
            else {return Some(ptr[0..i].to_string());}
        }
    }
    return Some(ptr.to_string());
}
fn current_token_bounds(command: &mut String, cursor_pos: &mut usize) -> (usize,usize, usize) { //Find beginning and ends of current token
    let mut token_begin = 0;
    let mut token_end = 0;
    let mut token_index = 0;

    let mut active_single_quotes = false; 
    let mut active_double_quotes = false;
    let mut i = 0;
    while i <= command.len() {
        let mut j: usize = i;
        token_begin = i;
        token_end = j;
        while j < command.len() {
            let character_j = command.as_bytes()[j] as char;

            if character_j == '\'' {
                if !active_double_quotes {active_single_quotes = !active_single_quotes; j+=1; continue;}
            }
            if character_j == '\"' {
                if !active_single_quotes {active_double_quotes = !active_double_quotes; j+=1; continue;}
            }

            //CHARACTER IF_ELSE
            if active_single_quotes {
            }
            else if active_double_quotes {
            }
            else {
                if character_j == ' ' {break;}
            }
            j+=1;
        }
        if j >= *cursor_pos {token_end = j; break;}
        i = j + 1;
        if (i < command.len() && command.as_bytes()[i] as char != ' ') {token_index+=1;}
        if j == command.len()-1 && command.as_bytes()[j] as char == ' ' {token_index+=1;}

    }
    return (token_begin, token_end, token_index)
}
pub fn to_replace(cmd_buffer: &mut String, cursor_pos: &mut usize, multi_output: &mut bool) { //Identify what to replace
    //IDENTIFY TOKEN TO AUTOCOMPLETE
    let (token_begin, token_end, token_index) = current_token_bounds(cmd_buffer, cursor_pos);
    let mut str_to_complete = cmd_buffer[token_begin..*cursor_pos].to_string();
    let replace_cmd = token_index == 0;
    match_command(&mut str_to_complete, multi_output, replace_cmd);
    cmd_buffer.replace_range(token_begin..token_end, &str_to_complete);
    *cursor_pos += str_to_complete.len()-(token_end - token_begin);
}
fn command_matches(prefix: &str) -> BTreeSet<String> { //Find matching commands
    let mut VecSt = BTreeSet::<String>::new();
    for cmd in inbuilt_commands.keys() {
        if cmd.starts_with(prefix) {
            VecSt.insert(format!("{} ", cmd.to_string()));
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
                                        VecSt.insert(format!("{} ", fname_str));
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
    // print!("{:?}", VecSt);
    VecSt
}
