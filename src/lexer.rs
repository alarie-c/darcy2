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
            let line: usize = 1;

            Self { tokens, chars, current, line, source }
        }

        // Iterates through every character in the source file and appends tokens to self.tokens vec
        // Does not return anything but does exhuast itself which sends the program back to main.rs
        // where the tokens are handled further
        pub fn scan(&mut self) {
            // Advance to first character
            if !self.advance() {
                self.end();
                // Return
            }

            // Loop is constant until EOF reached
            // where `break 'scanner;` is called, exhausting the method
            'scanner: loop {
                // Ignoring useless chars
                if self.current == ' ' || self.current == '\r' || self.current == '\t' {
                    // Advance the iterator
                    if self.advance() {
                        continue 'scanner;
                    }
                    self.end();
                    break 'scanner;
                }

                // Attempt to match to symbol
                let symbol = self.match_symbol();
                match symbol {
                    // If symbol match is found
                    LexRes::Match(token) => {
                        self.tokens.push(token);

                        // Advance the iterator
                        if self.advance() {
                            continue 'scanner;
                        }
                        self.end();
                        break 'scanner;
                    },

                    // If no symbol match found
                    LexRes::None => {
                        // Look for number literal
                        if self.current.is_numeric() {
                            let number = self.take_number();
                            match number {
                                // If we get a valid number literal back
                                LexRes::Match(number) => {
                                    let token = Token::new(TokenType::NumberLit, String::from(number), self.line);
                                    self.tokens.push(token);

                                    // Advance the iterator
                                    if self.advance() {
                                        continue 'scanner;
                                    }
                                    self.end();
                                    break 'scanner;
                                },

                                // If we don't get a valid number literal
                                LexRes::None => {},

                                // If EOF reached
                                LexRes::End => {
                                    self.end();
                                    break 'scanner;
                                }
                            }
                        }

                        // If not a number literal, try identifier/keyword
                        let alphanum = self.take_alphanum();
                        match alphanum {
                            // Attempt to match to keyword or id
                            LexRes::Match(result) => {
                                let keyword = self.match_keyword(&result);
                                match keyword {
                                    LexRes::Match(token) => {
                                        self.tokens.push(token);
                                        
                                        // Advance the iterator
                                        if self.advance() {
                                            continue 'scanner;
                                        }
                                        self.end();
                                        break 'scanner;
                                    },
                                    LexRes::None => {
                                        todo!("Match ID");
                                    },
                                    LexRes::End => {
                                        self.end();
                                        break 'scanner;
                                    }
                                }
                            },

                            // If no valid identifier is found
                            LexRes::None => {
                                todo!("Error handle no alphanum found");
                            },

                            // If EOF reached
                            LexRes::End => {
                                self.end();
                                break 'scanner;
                            }
                        }
                    },

                    // If EOF found
                    LexRes::End => {
                        self.end();
                        break 'scanner;
                    }
                }
            }
        }
        
        // Starts from current and takes every character until it reaches
        // a non number. Employs exceptions for _ to improve readibility (e.g. 100_000)
        // and . for floating point numbers
        fn take_number(&mut self) -> LexRes<String> {
            let mut buffer = String::new();

            'search: loop {
                if self.current.is_numeric() {
                    // Add number to buffer
                    buffer.push(self.current);

                    // Advance the iterator
                    if self.advance() {
                        continue 'search;
                    }
                    return LexRes::End;
                }

                // If not alphanumeric, match it to one of the exceptions
                match self.current {
                    '_' => {                   
                        // Advance the iterator
                        if self.advance() {
                            continue 'search;
                        }
                        return LexRes::End;
                    },
                    '.' => {
                        // Add . to literal
                        buffer.push('.');
                        
                        // Advance the iterator
                        if self.advance() {
                            continue 'search;
                        }
                        return LexRes::End;
                    },
                    _ => return LexRes::Match(buffer),
                }
            }
        }

        // Starts from current and advances until it reaches a ",
        // signifying the termination of the string literal
        // This method returns only the contents, not the " chars
        fn take_string(&mut self) -> LexRes<String> {
            let mut buffer = String::new();

            'search: loop {
                // If character is the terminating " 
                if self.current == '"' {
                    // Determine if buffer is empty
                    return LexRes::Match(buffer);
                }

                // Otherwise, add the character to the buffer
                buffer.push(self.current);

                // Advance the iterator
                if self.advance() {
                    continue 'search;
                }
                return LexRes::End;
            }
        }

        // Starts from current and takes every alphanumeric character until
        // it finds one that isn't alphanumeric, where it returns the chars it
        // has collected in `buffer`
        fn take_alphanum(&mut self) -> LexRes<String> {
            let mut buffer = String::new();
            
            'search: loop {
                if self.current.is_alphanumeric() {
                    // Add char to buffer if alphanum
                    buffer.push(self.current);
                    
                    // Advance the iterator
                    if self.advance() {
                        continue 'search;
                    }
                    return LexRes::End;

                } 
                match self.current {
                    // Make exception for underscores
                    '_' => {
                        // Add char to buffer if _
                        buffer.push(self.current);

                        // Advance the iterator
                        if self.advance() {
                            continue 'search;
                        }
                        return LexRes::End;
                    },
                    
                    // If character is not _ or alphanum, return buffer
                    _ => {
                        // If empty, return none, otherwise return buffer
                        if !buffer.is_empty() {
                            return LexRes::Match(buffer);
                        } else {
                            return LexRes::None;
                        } 
                    },
                }
            }
        }

        // Takes a given input and attempts to match it to a string
        // Returns a LexRes enum with the attached token if successful
        fn match_keyword(&self, string: &str) -> LexRes<Token> {
            match string {
                "cout" => LexRes::Match(Token::new(TokenType::Cout, String::from("cout"), self.line)),
                _ => LexRes::None,
            }
        }

        // Attempts to match the current character to a set of tokens
        // This method also employs look ahead for matching multiple char symbols like `==`
        // Returns a LexRes enum with the attached token if successful
        fn match_symbol(&mut self) -> LexRes<Token> {
            match self.current {
                '=' => LexRes::Match(Token::new(TokenType::Equals, String::from("="), self.line)),
                '!' => LexRes::Match(Token::new(TokenType::Bang, String::from("!"), self.line)),
                '+' => LexRes::Match(Token::new(TokenType::Plus, String::from("+"), self.line)),
                '-' => LexRes::Match(Token::new(TokenType::Minus, String::from("-"), self.line)),
                '/' => LexRes::Match(Token::new(TokenType::Slash, String::from("/"), self.line)),
                '*' => LexRes::Match(Token::new(TokenType::Star, String::from("*"), self.line)),

                // String literal
                '"' => {
                    if self.advance() {
                        let string = self.take_string();
                        match string {
                            // If a string literal is found
                            LexRes::Match(literal) => return LexRes::Match(Token::new(TokenType::StringLit, literal, self.line)),
                            
                            // If the string literal is empty
                            LexRes::None => {
                                todo!("Error handling  string or something ig");
                            },

                            // If EOF reached
                            LexRes::End => return LexRes::End,
                        }
                    }

                    eprintln!("Error handle here, non-terminating literal");
                    LexRes::End
                }

                // Newline
                '\n' => {
                    self.line += 1;
                    LexRes::Match(Token::new(TokenType::NewLn, String::from("newline"), self.line - 1))
                }

                // No matches found
                _ => LexRes::None,
            }
        }

        // Pushes a new EOF token to the end of the tokens vec in Lexer
        // Simply used as a way to simplify called the termination of the parser
        // rather than typing this all out every single time a helper
        // method returns `LexRes::End``
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
                return true;
            }
            false
        }
    }
}