pub mod lexer {
    use std::iter::Peekable;
    use crate::{Token, TokenType};

    // Lexer result handles interfacing between self.scan() and
    // the various helper functions it calls
    enum LexRes<Token> {
        Match(Token), // Found a token
        None, // Could not find a valid token
        End, // Found end of file
    }

    pub struct Lexer<Iter: Iterator<Item = char>> {
        pub tokens: Vec<Token>,
        chars: Peekable<Iter>,
        current: char,
        line: usize,
        pub source: Vec<String>,
    }

    impl <Iter: Iterator<Item = char>> Lexer<Iter> {
        // Construct a new instance of Lexer with iter and 
        pub fn new(chars: Peekable<Iter>, source: Vec<String>) -> Self {
            let tokens = Vec::new();
            let current = ' ';
            let line = 1 as usize;

            Self { tokens, chars, current, line, source }
        }

        pub fn scan(&mut self) {
            // Advance to first character
            if !self.advance() {
                self.end();
                // Return
            }

            'scanner: loop {
                // Ignoring useless chars
                if self.current == ' ' || self.current == '\r' || self.current == '\t' {
                    if self.advance() {
                        continue 'scanner;
                    } else {
                        self.end();
                        break 'scanner;
                    }
                }

                // Attempt to match to symbol
                let symbol = self.match_symbol();
                match symbol {
                    // If symbol match is found
                    LexRes::Match(token) => {
                        self.tokens.push(token);

                        if self.advance() {
                            continue 'scanner;
                        } else {
                            self.end();
                            break 'scanner;
                        }
                    },

                    // If no symbol match found
                    LexRes::None => {
                        todo!("Match IDs");
                    },

                    // If EOF found
                    LexRes::End => {
                        self.end();
                        break 'scanner;
                    }
                }
            }
        }

        fn match_symbol(&mut self) -> LexRes<Token> {
            match self.current {
                '=' => LexRes::Match(Token::new(TokenType::EQUALS, String::from("="), self.line)),
                '!' => LexRes::Match(Token::new(TokenType::BANG, String::from("!"), self.line)),
                '+' => LexRes::Match(Token::new(TokenType::PLUS, String::from("+"), self.line)),
                '-' => LexRes::Match(Token::new(TokenType::MINUS, String::from("-"), self.line)),
                '/' => LexRes::Match(Token::new(TokenType::SLASH, String::from("/"), self.line)),
                '*' => LexRes::Match(Token::new(TokenType::STAR, String::from("*"), self.line)),

                // Newline
                '\n' => {
                    self.line += 1;
                    LexRes::Match(Token::new(TokenType::NEWLN, String::from("newline"), self.line - 1))
                }

                // No matches found
                _ => LexRes::None,
            }
        }

        fn end(&mut self) {
            let end_token = Token::end(self.line);
            self.tokens.push(end_token);
        }
        
        // Helper function returns true if can advance
        // Sets self.current to updated character
        // Returns false if EOF
        fn advance(&mut self) -> bool {
            if let Some(c) = self.chars.next() {
                self.current = c;
                true
            } else {
                false
            }
        }
    }
}