use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::token::{self, TokenKind};

const EOF_CHAR: char = 0xFF as char;

pub struct Lexer {
    reader: BufReader<File>,
    current_line: u32,
    current_char: char,
}

impl Lexer {
    pub fn new(input_file: String) -> Lexer {
        let input = File::open(input_file);
        let reader = BufReader::new(input.unwrap());
        Lexer {
            reader,
            current_line: 0,
            current_char: ' ',
        }
    }

    /// consumes the next character in the file. 
    /// 
    /// If EOF is reached, returns nothing. 
    fn advance(&mut self) {
        return match self.reader.fill_buf() {
            Ok(buffer) if (buffer.len() > 0)  => {
                let tmp = buffer[0] as char;
                self.reader.consume(1);
                self.current_char = tmp;
            }
            _ => {
                self.current_char = EOF_CHAR;
            }
        };
    }

    pub fn lex(&mut self) -> token::TokenKind {
        self.skip_whitespace();
        
        match self.current_char {
            EOF_CHAR => TokenKind::EOF,
            '(' => {
                self.advance();
                TokenKind::LeftParen
            },
            ')' => {
                self.advance();
                TokenKind::RightParen
            },
            '{' => {
                self.advance();
                TokenKind::LeftBrace
            },
            '}' => {
                self.advance();
                TokenKind::RightBrace
            }
            '[' => {
                self.advance();
                TokenKind::LeftBracket
            },
            ']' => {
                self.advance();
                TokenKind::RightBracket
            },
            '~' => {
                self.advance();
                TokenKind::BitNotOp
            },
            ';' => {
                self.advance();
                TokenKind::Semicolon
            },
            ',' => {
                self.advance();
                TokenKind::Comma
            },
            '+' => {
                self.advance();
                match self.current_char {
                    '+' => {
                        self.advance();
                        TokenKind::IncrementOp
                    },
                    '=' => {
                        self.advance();
                        TokenKind::PlusEqOp
                    },
                    _ => TokenKind::PlusOp,
                }
            }
            '-' => {
                self.advance();
                match self.current_char {
                    '-' => {
                        self.advance();
                        TokenKind::DecrementOp
                    },
                    '=' => {
                        self.advance();
                        TokenKind::MinusEqOp
                    },
                    _ => TokenKind::MinusOp,
                }
            }
            '.' => {
                self.advance();
                match self.current_char {
                    '.' => {
                        self.advance();
                        if self.current_char != '.' {
                            panic!("ERROR: Unknown symbol found on line {}", self.current_line);
                        }
                        self.advance();
                        TokenKind::Elipses
                    }
                    '0'..='9' => { // floats that are < 1 without a leading 0.
                        TokenKind::FloatLiteral(self.handle_decimal())
                    }
                    _ => TokenKind::Dot,
                }
            }
            _ => unreachable!()
        }
    }

    /// Returns the float value of the decimal part of a float.
    fn handle_decimal(&mut self) -> f32 {
        let mut float_value = vec!['0', '.', self.current_char];
        self.advance();
        while self.current_char.is_digit(10) {
            float_value.push(self.current_char);
            self.advance();
        }
        float_value
                .into_iter()
                .collect::<String>()
                .parse::<f32>()
                .unwrap()
    }

    /// Advances until the current_char is not a whitespace character.
    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            if self.current_char == '\n' {
                self.current_line += 1;
            }
            self.advance();
        }
    }
}