use std::{
    io::{self, stderr, stdout, Write},
    process::Stdio,
};

#[allow(non_camel_case_types)]
pub struct out_stream {
    pub stdout: Output,
    pub stderr: Output,
}

pub enum Output {
    Stdout,
    Stderr,
    File(std::fs::File),
}

impl From<&Output> for Stdio {
    fn from(s: &Output) -> Self {
        match s {
            Output::Stdout => Stdio::inherit(),
            Output::Stderr => Stdio::inherit(),
            Output::File(file) => Stdio::from(file.try_clone().unwrap()),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Stdout => stdout().write(buf),
            Output::Stderr => stderr().write(buf),
            Output::File(file) => file.write(buf),
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
            Output::File(file) => Output::File(file.try_clone().unwrap()),
        }
    }
}
