use std::io::{self, Write};
use termios::*;
use std::io::Read;
use crate::terminal::completion::*;
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
pub fn terminal_read(buffer: &mut String) { //Continuously process input
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
                to_replace(buffer, &mut cursor, &mut multi_output);
                // buffer.push(' ');
                // cursor = buffer.len();
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
        io::stdout().flush();
        buffer.pop();
        if buffer.ends_with('\x07') {buffer.pop();}
        io::stdout().flush();
    }
}