pub mod token {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenType {
        // Binary Operators
        Plus,
        Minus,
        Star,
        Slash,
        Percent,

        // Symbols
        Equals,
        Bang,

        // Literals
        StringLit,
        NumberLit,
        Type,

        // Keywords
        Cout,

        // Other
        NewLn,
        EndFile,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub line: usize,
    }

    impl Token {
        pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
            Self { token_type, lexeme, line }
        }

        pub fn end(line: usize) -> Self {
            let token_type = TokenType::EndFile;
            let lexeme = String::from("<-- END OF FILE -->");
            
            Self { token_type, lexeme, line } 
        }
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}\n\t{}: {}", self.token_type, self.line, self.lexeme)
        }
    }
}