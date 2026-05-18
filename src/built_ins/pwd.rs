use crate::exec::output::out_stream;
use std::{env, io::Write};

pub fn print_working_dir(
    _arg_array: Vec<String>,
    mut output_stream: out_stream,
) -> Result<i32, String> {
    writeln!(output_stream.stdout, "{}", env::current_dir().unwrap().display());
    Ok(1)
}
