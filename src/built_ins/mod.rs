use crate::exec::output::out_stream;
use std::{collections::HashMap, sync::LazyLock};

pub mod cd;
pub mod echo;
pub mod exit;
pub mod pwd;
pub mod type_cmd;

pub use cd::change_working_directory;
pub use echo::echo;
pub use exit::exit_program;
pub use pwd::print_working_dir;
pub use type_cmd::type_function;

#[allow(non_camel_case_types)]
pub type BuiltinHandler = fn(Vec<String>, out_stream) -> Result<i32, String>;

#[allow(non_upper_case_globals)]
pub static inbuilt_commands: LazyLock<HashMap<&'static str, BuiltinHandler>> = LazyLock::new(|| {
    HashMap::from([
        ("exit", exit_program as BuiltinHandler),
        ("echo", echo as BuiltinHandler),
        ("type", type_function as BuiltinHandler),
        ("pwd", print_working_dir as BuiltinHandler),
        ("cd", change_working_directory as BuiltinHandler),
    ])
});
