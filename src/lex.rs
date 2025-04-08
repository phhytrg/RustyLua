use std::{
    fs::File,
    io::{Read, Seek},
};

#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
}

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Self { input }
    }

    pub fn next(&mut self) -> Option<Token> {
        let ch = self.read_char();
        match ch {
            ' ' | '\r' | '\n' | '\t' => self.next(),
            '\0' => None,
            '"' => {
                // String, read the whole string
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        // c if c.is_control() => panic!("Invalid token"),
                        '"' => return Some(Token::String(s)),
                        c => s.push(c),
                    }
                }
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        c if c.is_alphanumeric() || c == '_' => name.push(c),
                        _ => {
                            self.input.seek(std::io::SeekFrom::Current(-1)).unwrap();
                            return Some(Token::Name(name));
                        }
                    }
                }
            }
            ch => panic!("Invalid token: {:?}", ch),
        }
    }

    fn read_char(&mut self) -> char {
        let mut buf = [0u8; 1];
        match self.input.read_exact(&mut buf) {
            Ok(()) => buf[0] as char,
            Err(_) => '\0',
        }
    }
}
