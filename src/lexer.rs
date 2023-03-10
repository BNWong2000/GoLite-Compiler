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
            '*' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::StarEqOp
                    }
                    _ => {
                        TokenKind::StarOp
                    }
                }
            }
            '/' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::DivEqOp
                    }
                    '/' => {
                        while self.current_char != '\n' {
                            self.advance();
                        }
                        TokenKind::Comment
                    }
                    '*' => {
                        let mut star_found = false;
                        loop {
                            self.advance();
                            if star_found && self.current_char == '/' {
                                break;
                            }
                            star_found = self.current_char == '*';
                        }
                        TokenKind::Comment
                    }
                    _ => {
                        TokenKind::DivOp
                    }
                }
            }
            '%' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::ModEqOp
                    }
                    _ => {
                        TokenKind::ModOp
                    }
                }
            }
            '=' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::LogicEqOp
                    }
                    _ => {
                        TokenKind::EqOp
                    }
                }
            }
            ':' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::AssignOp
                    }
                    _ => {
                        TokenKind::Colon
                    }
                }
            }
            '&' => {
                self.advance();
                match self.current_char {
                    '&' => {
                        self.advance();
                        TokenKind::LogicAndOp
                    }
                    '=' => {
                        self.advance();
                        TokenKind::BitAndEqOp
                    }
                    '^' => {
                        self.advance();
                        if self.current_char == '=' {
                            self.advance();
                            TokenKind::BitClearEqOp
                        } else {
                            TokenKind::BitClearOp
                        }
                    }
                    _ => {
                        TokenKind::BitAndOp
                    }
                }
            }
            '|' => {
                self.advance();
                match self.current_char {
                    '|' => {
                        self.advance();
                        TokenKind::LogicOrOp
                    }
                    '=' => {
                        self.advance();
                        TokenKind::BitOrEqOp
                    }
                    _ => {
                        TokenKind::BitOrOp
                    }
                }
            }
            '^' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::BitXorEqOp
                    }
                    _ => {
                        TokenKind::BitXorOp
                    }
                }
            }
            '!' => {
                self.advance();
                match self.current_char {
                    '=' => {
                        self.advance();
                        TokenKind::LogicNotEqOp
                    }
                    _ => {
                        TokenKind::LogicNotOp
                    }
                }
            }
            '<' => self.handle_less_than(),
            '>' => self.handle_greater_than(),
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
            '\"' => self.handle_strings(),
            '\'' => self.handle_char(),
            _ => {
                if self.current_char.is_ascii_alphabetic() || self.current_char == '_' {
                    self.handle_identifers_and_keywords()
                } else if self.current_char.is_digit(10) {
                    self.handle_number()
                }else {
                    panic!("ERROR: Unknown character found on line {}", self.current_line);
                }
            }
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

    fn handle_less_than(&mut self) -> TokenKind {
        self.advance();
        match self.current_char {
            '-' => {
                self.advance();
                TokenKind::DirectionOp
            }
            '<' => {
                self.advance();
                if self.current_char == '=' {
                    self.advance();
                    TokenKind::LShiftEqOp
                } else {
                    TokenKind::LShiftOp
                }
            }
            '=' => {
                self.advance();
                TokenKind::LogicLessEqOp
            }
            _ => {
                TokenKind::LogicLessOp
            }
        }
    }

    fn handle_greater_than(&mut self) -> TokenKind {
        self.advance();
        match self.current_char {
            '<' => {
                self.advance();
                if self.current_char == '=' {
                    self.advance();
                    TokenKind::RShiftEqOp
                } else {
                    TokenKind::RShiftOp
                }
            }
            '=' => {
                self.advance();
                TokenKind::LogicGreaterEqOp
            }
            _ => {
                TokenKind::LogicGreaterOp
            }
        }
    }

    fn handle_strings(&mut self) -> TokenKind {
        let mut string_result: Vec<char> = vec![];
        self.advance();
        while self.current_char != '\"' {
            if self.current_char == '\\' {
                string_result.push('\\');
                self.advance();
                if !self.is_valid_escape_sequence_char(self.current_char) {
                    panic!("ERROR: Invalid escape character sequence");
                }
            } else if self.is_escape(self.current_char) {
                panic!("ERROR: Escape character inside of string (perhaps you want to use a valid escape sequence instead?)");
            }
            string_result.push(self.current_char);
            self.advance()
        }
        self.advance();
        TokenKind::StringLiteral(string_result.into_iter().collect::<String>().to_string())
    }
 
    fn handle_char(&mut self) -> TokenKind {
        self.advance();
        let mut char_result: Vec<char> = vec![];
        if self.current_char == '\'' {
            panic!("ERROR: empty character");
        }
        if self.current_char == '\\' {
            char_result.push('\\');
            self.advance();
            if !self.is_valid_escape_sequence_char(self.current_char) {
                panic!("ERROR: Invalid escape character sequence");
            }
        } else if self.is_escape(self.current_char) {
            panic!("ERROR: Escape character inside of char (perhaps you want to use a valid escape sequence instead?)");
        }
        char_result.push(self.current_char);
        self.advance();
        if self.current_char != '\'' {
            panic!("ERROR: invalid char literal");
        }
        self.advance();
        TokenKind::CharLiteral(char_result.into_iter().collect::<String>().to_string())
    }

    fn is_valid_escape_sequence_char(&mut self, c: char) -> bool {
        if c == 'a' || c == 'b' || c == 'f' || c == 'n'
                || c == 'r' || c == 't' || c == 'v' || c == '\\'
                || c == '\'' || c == '\"' {
            true
        } else {
            false
        }
    }

    fn handle_number(&mut self) -> TokenKind {
        let mut number = String::new();
        number.push(self.current_char);
        self.advance();
        while self.current_char.is_digit(10) {
            number.push(self.current_char);
            self.advance();
        }
        let int_result = number.parse::<i32>().unwrap();
        if self.current_char == '.' {
            self.advance();
            let mut float_result = int_result as f32;
            if self.current_char.is_digit(10) {
                let decimal_part = self.handle_decimal();
                float_result = float_result + decimal_part;
            }
            TokenKind::FloatLiteral(float_result)
        } else {
            TokenKind::IntLiteral(int_result)
        }
    }

    fn handle_identifers_and_keywords(&mut self) -> TokenKind {
        let mut word = String::new();
        word.push(self.current_char);
        self.advance();
        while self.current_char.is_ascii_alphanumeric() || self.current_char == '_' {
            word.push(self.current_char);
            self.advance();
        }
        match word.as_str() {
            "break" => TokenKind::BreakKeyword,
            "default" => TokenKind::DefaultKeyword,
            "func" => TokenKind::FuncKeyword,
            "interface" => TokenKind::InterfaceKeyword,
            "select" => TokenKind::SelectKeyword,
            "case" => TokenKind::CaseKeyword,
            "defer" => TokenKind::DeferKeyword,
            "go" => TokenKind::GoKeyword,
            "map" => TokenKind::MapKeyword,
            "struct" => TokenKind::StructKeyword,
            "chan" => TokenKind::ChanKeyword,
            "else" => TokenKind::ElseKeyword,
            "goto" => TokenKind::GotoKeyword,
            "package" => TokenKind::PackageKeyword,
            "switch" => TokenKind::SwitchKeyword,
            "const" => TokenKind::ConstKeyword,
            "fallthrough" => TokenKind::FallThroughKeyword,
            "if" => TokenKind::IfKeyword,
            "range" => TokenKind::RangeKeyword,
            "type" => TokenKind::TypeKeyword,
            "continue" => TokenKind::ContinueKeyword,
            "for" => TokenKind::ForKeyword,
            "import" => TokenKind::ImportKeyword,
            "return" => TokenKind::ReturnKeyword,
            "var" => TokenKind::VarKeyword,
            _ => TokenKind::Identifier(word)
        }
    }

    fn is_escape(&mut self, c: char) -> bool {
        let num_char = c as u8;
        // Rust doesn't recognize some escapes like '\f' so I had to hard code them as u8's
        if num_char == 0x07 || num_char == 0x08
                || num_char == 0x0c || num_char == 0x0a
                || num_char == 0x0d || num_char == 0x09
                || num_char == 0x0b || num_char == 0x22
                || num_char == 0x27 || num_char == 0x5c {
            true
        } else {
            false
        }
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

#[cfg(test)]
pub mod lexer_test {
    use crate::token::TokenKind;
    /// TODO: Need to properly check these...
    #[test]
    pub fn given_valid_tokens_should_output_token_strings() {
        let file_name = "testfiles/all_tokens.test".to_string();
        let mut lexer = super::Lexer::new(file_name);
        let mut curr_token = lexer.lex();
        while curr_token != TokenKind::EOF {
            print!("{}\n", curr_token.to_string() );
            curr_token = lexer.lex();
        }
    }

    #[test]
    pub fn given_valid_strings_and_chars_should_output_correct_tokens() {
        let file_name = "testfiles/strings_and_chars.test".to_string();
        let mut lexer = super::Lexer::new(file_name);
        let mut curr_token = lexer.lex();
        while curr_token != TokenKind::EOF {
            print!("{}\n", curr_token.to_string() );
            curr_token = lexer.lex();
        }
    }

    #[test]
    #[should_panic]
    pub fn given_invalid_strings_and_chars_should_panic() {
        let file_name = "testfiles/illegal_strings_and_chars.test".to_string();
        let mut lexer = super::Lexer::new(file_name);
        let mut curr_token = lexer.lex();
        while curr_token != TokenKind::EOF {
            print!("{}\n", curr_token.to_string() );
            curr_token = lexer.lex();
        }
    }

    #[test]
    #[should_panic]
    pub fn given_invalid_tokens_should_panic() {
        let file_name = "testfiles/illegal_tokens.test".to_string();
        let mut lexer = super::Lexer::new(file_name);
        let mut curr_token = lexer.lex();
        while curr_token != TokenKind::EOF {
            print!("{}\n", curr_token.to_string() );
            curr_token = lexer.lex();
        }
    }
}