use std::fs::File;
use std::io::{BufReader, BufRead};
use std::char::{};
use crate::token;

pub struct Lexer {
    reader: BufReader<File>,
}

impl Lexer {
    pub fn new(input_file: String) -> Lexer {
        let input = File::open(input_file);
        let mut reader = BufReader::new(input.unwrap());
        Lexer {
            reader
        }
    }

    /// consumes the next character in the file. 
    /// 
    /// If EOF is reached, returns nothing. 
    fn advance(&mut self) -> Option<char> {
        return match self.reader.fill_buf() {
            Ok(buffer) if (buffer.len() > 0)  => {
                let tmp = buffer[0] as char;
                self.reader.consume(1);
                Some(tmp)
            }
            _ => {
                None
            }
        };
    }

    // pub fn lex(&mut self) -> token::Token_kind{

    // }
}