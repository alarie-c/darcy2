pub mod lexer {
    use std::iter::Peekable;
    use crate::{Token, TokenType};

    pub struct Lexer<Iter: Iterator<Item = char>> {
        pub tokens: Vec<Token>,
        chars: Peekable<Iter>,
        current: char,
        line: usize,
        source: Vec<String>,
        scanning: bool,
    }

    impl <Iter: Iterator<Item = char>> Lexer<Iter> {
        // Construct a new instance of Lexer with iter and 
        pub fn new(chars: Peekable<Iter>, source: Vec<String>) -> Self {
            let tokens = Vec::new();
            let current = ' ';
            let line = 1 as usize;
            let scanning = false;

            Self { tokens, chars, current, line, source, scanning }
        }

        pub fn scan(&mut self) {

        }
    }
}