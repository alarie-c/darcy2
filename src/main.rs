use std::{env, io::Read, fs::File};
use lexer::lexer::Lexer;
use token::token::{Token, TokenType};

mod token;
mod lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

    // Get file and flags
    if args.len() >= 2 {        
        // Get file and make buffer
        let mut file = File::open(&args[1]).expect("Error reading file!");
        let mut file_buffer = String::new();
        file.read_to_string(&mut file_buffer).expect("Error");

        // Create iterator for file
        let mut lines: Vec<String> = Vec::new();
        let chars = file_buffer.chars();

        // Take characters until new line
        let mut line_buffer = String::new();
        for c in chars {
            if c == '\n' {
                lines.push(line_buffer);
                line_buffer = String::new();
            } else {
                line_buffer.push(c);
            }
        }
        lines.push(line_buffer);

        let peekable_chars = file_buffer.chars().peekable();
        let mut lexer = Lexer::new(peekable_chars, lines);
        lexer.scan();

        println!("{:#?}", lexer.tokens);

        // println!("{lines:#?}");
        // println!("{}", lines.len());

    } else {
        eprintln!("File path not specified.");
        std::process::exit(1);
    }
}
