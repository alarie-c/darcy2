use std::{env, io::Read, fs::File};
use lexer::lexer::Lexer;
use token::token::{Token, TokenType};

mod token;
mod lexer;
mod ast;


fn main() {
    let args: Vec<String> = env::args().collect();

    // Get file and flags
    if args.len() >= 2 {        
        // Get file and make buffer
        let mut file = File::open(&args[1]).expect("Error reading file!");
        let mut file_buffer = String::new();
        file.read_to_string(&mut file_buffer).expect("Error");

        let lines: Vec<String> = file_buffer.split("\n").map(|x| x.to_string()).collect();
        let chars = file_buffer.chars().peekable();
        
        let mut lexer = Lexer::new(chars, lines);
        lexer.scan();

        println!("{:#?}", lexer.tokens);

        // Print each token and it's corresponding line content
        // for token in lexer.tokens {
        //     println!("{:?}", token);

        //     // Find the line content
        //     let line_content = &lexer.source[token.line - 1];
        //     println!("{line_content}");
        // }

        // println!("{lines:#?}");
        // println!("{}", lines.len());

    } else {
        eprintln!("File path not specified.");
        std::process::exit(1);
    }
}
