use crate::exec::output::out_stream;
use std::{env, io::Write};

pub fn change_working_directory(
    arg_array: Vec<String>,
    mut output_stream: out_stream,
) -> Result<i32, String> {
    let newdir = match arg_array.get(1) {
        Some(dir) => dir.clone(),
        None => match env::var("HOME") {
            Ok(home) => home,
            Err(_) => return Err("No directory".to_string()),
        },
    };

    match env::set_current_dir(&newdir) {
        Ok(_) => Ok(0),
        Err(_) => {
            writeln!(
                output_stream.stderr,
                "{}: No such file or directory",
                newdir
            );
            Err("directory doesn't exist".to_string())
        }
    }
}
