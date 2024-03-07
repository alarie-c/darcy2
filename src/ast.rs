mod node {

    #[derive(Debug, Clone, PartialEq)]
    pub enum Node {
        BinaryExpr(BinaryExpr),

        // Literals
        StringLitExpr(LiteralExpr),
        NumberLitExpr(LiteralExpr),
        IntegerLitExpr(LiteralExpr),

    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum BinaryOp {
        Plus,
        Minus,
        Multiply,
        Divide,
        Modulus,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LiteralType {
        String(String),
        Number(f64),
        Integer(i32),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BinaryExpr {
        pub op: BinaryOp,
        pub ln: Box<Node>,
        pub rn: Box<Node>,
    }   

    #[derive(Debug, Clone, PartialEq)]
    pub struct LiteralExpr {
        pub typ: LiteralType,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RootNode {
        pub children: Vec<Node>,
    }

    impl RootNode {
        pub fn push_node(&mut self, node: Node) {
            self.children.push(node);
        }
    }
}

pub mod ast {
    use crate::token::token::{Token, TokenType};
    use super::node::{LiteralExpr, LiteralType, Node, RootNode};

    pub struct Ast {
        pub tokens: Vec<Token>,
        pub root: Box<RootNode>,
    }

    impl Ast {
        // Constructs a new AST struct with the tokens from the lexer
        // and an empty root node
        pub fn new(tokens: Vec<Token>) -> Self {
            let root = Box::new(RootNode { children: Vec::new() });
            
            Self {
                tokens,
                root,
            }
        }

        pub fn construct(&mut self) {
            let mut idx: usize = 0;
            
            'construct: while idx <= self.tokens.len() {
                // Set current token
                let current = &self.tokens[idx];

                match current.token_type {
                    // Number literals
                    TokenType::NumberLit => {
                        // Check if number literal is a floating point num
                        if current.lexeme.contains(".") {
                            let parsed_num = match current.lexeme.parse::<f64>() {
                                Ok(parsed_num) => parsed_num,
                                Err(_) => {
                                    eprintln!("Error parsing number literal to float");
                                    todo!("Error handling here");
                                }
                            };
                            
                            let literal = LiteralExpr {
                                typ: LiteralType::Number(parsed_num),
                            };

                            let node = Node::NumberLitExpr(literal);
                            self.root.push_node(node);
                            
                            idx += 1;
                            continue 'construct;
                        }

                        // Otherwise, make integer
                        let parsed_num = match current.lexeme.parse::<i32>() {
                            Ok(parsed_num) => parsed_num,
                            Err(_) => {
                                eprintln!("Error parsing number literal to float");
                                todo!("Error handling here");
                            }
                        };
                        
                        let literal = LiteralExpr {
                            typ: LiteralType::Integer(parsed_num),
                        };

                        let node = Node::NumberLitExpr(literal);
                        self.root.push_node(node);

                        idx += 1;
                        continue 'construct;
                    },

                    // String literals
                    TokenType::StringLit => {
                        let literal: LiteralExpr = LiteralExpr {
                            typ: LiteralType::String(String::from(&current.lexeme))
                        };

                        let node = Node::StringLitExpr(literal);
                        self.root.push_node(node);

                        idx += 1;
                        continue 'construct;
                    },

                    // End of file
                    TokenType::EndFile => break 'construct,

                    _ => {
                        idx += 1;
                        continue 'construct;
                    },
                }
            }
        }
    }
}