pub mod token {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenType {
        // Binary Operators
        Plus,
        Minus,
        Slash,
        Star,
        Modulus,

        // Symbols
        Equals,
        Bang,

        // Literals
        StringLiteral,

        // Keywords
        Cout,

        // Other
        NL,
        EOF,
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
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}\n\t{}: {}", self.token_type, self.line, self.lexeme)
        }
    }
}