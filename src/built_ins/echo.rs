use crate::exec::output::out_stream;
use std::io::Write;

pub fn echo(arg_array: Vec<String>, mut output_stream: out_stream) -> Result<i32, String> {
    for i in 1..arg_array.len() {
        if i > 1 {
            write!(output_stream.stdout, " ");
        }
        write!(output_stream.stdout, "{}", arg_array[i]);
    }
    write!(output_stream.stdout, "\n");
    Ok(0)
}
