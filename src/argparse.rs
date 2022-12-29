use clap::{Arg, App};

pub struct Arguments {
    pub input_file: String,     // input file
    pub output_file: String,    // output file
    pub print_ast: bool,        // flag to show ast.
}

impl Arguments{
    pub fn new() -> Arguments{
        let default_output_file = "a.s";

        let matches = App::new("Compiler")
                .version("1.0")
                .author("BNWong")
                .about("A Compiler Project")
                .arg(Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .required(true)
                        .takes_value(true)
                        .help("Input file path"))
                .arg(Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .required(false)
                        .takes_value(true)
                        .help("Output file path"))
                .arg(Arg::with_name("ast")
                        .short("a")
                        .long("ast")
                        .required(false)
                        .takes_value(false)
                        .help("Flag to print the AST"))
                .get_matches();
        
        let input_file = matches.value_of("input").unwrap();
        let output_file = matches.value_of("output").unwrap_or(default_output_file);
        let print_ast = matches.is_present("ast");

        Arguments { 
            input_file: input_file.to_string(), 
            output_file: output_file.to_string(), 
            print_ast: print_ast 
        }
    }
}