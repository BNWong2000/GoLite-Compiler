use crate::lexer;
pub struct Parser {
    lexer: lexer::Lexer
}

impl Parser {
    pub fn new(input_lexer: lexer::Lexer) -> Parser {
        Parser {
            lexer: input_lexer,
        }
    }
}