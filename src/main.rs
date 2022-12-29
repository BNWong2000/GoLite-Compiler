pub mod argparse;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() {
    let options = argparse::Arguments::new();
    let lexer = lexer::Lexer::new(options.input_file);
    let parser = parser::Parser::new(lexer);
}
