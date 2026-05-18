use crate::exec::output::out_stream;
use std::{env, fs::metadata, io::Write, os::unix::fs::PermissionsExt};

use super::inbuilt_commands;

fn path_finder(executable_name: &str) -> Result<String, bool> {
    let val = match env::var("PATH") {
        Ok(path_dir) => path_dir,
        Err(_) => "".to_string(),
    };
    let mut vector_of_paths = val.split(':');
    let mut path_iterator = vector_of_paths.next();

    loop {
        match path_iterator {
            Some(directory) => {
                let file_path = format!("{}/{}", directory, executable_name);
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
            }
            None => break,
        }

        path_iterator = vector_of_paths.next();
    }

    Err(false)
}

pub fn type_function(
    arg_array: Vec<String>,
    mut output_stream: out_stream,
) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        let command_to_search = arg_array.get(i);
        match command_to_search {
            Some(command_to_search) => {
                if inbuilt_commands.contains_key(command_to_search.as_str()) {
                    writeln!(
                        output_stream.stdout,
                        "{} is a shell builtin",
                        command_to_search
                    );
                    continue;
                }

                match path_finder(command_to_search) {
                    Ok(val) => {
                        writeln!(
                            output_stream.stdout,
                            "{} is {}/{}",
                            command_to_search,
                            val,
                            command_to_search
                        );
                        return Ok(0);
                    }
                    Err(_) => {}
                };

                writeln!(output_stream.stderr, "{}: not found", command_to_search);
            }
            None => {}
        }
    }

    Ok(0)
}
