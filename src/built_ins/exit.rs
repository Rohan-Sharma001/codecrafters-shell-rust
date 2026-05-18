use crate::exec::output::out_stream;

pub fn exit_program(
    _arg_array: Vec<String>,
    mut _output_stream: out_stream,
) -> Result<i32, String> {
    Ok(-1)
}
