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
    use super::node::{BinaryExpr, BinaryOp, LiteralExpr, LiteralType, Node, RootNode};

    enum AstRes<T> {
        Node(T), // Created a new node
        None, // Could not make a valid node
        End, // Found end of file
    }

    pub struct Ast {
        pub stream: Vec<Token>,
        pub root: Box<RootNode>,
        idx: usize,
    }

    impl Ast {
        // Constructs a new AST struct with the tokens from the lexer
        // and an empty root node
        pub fn new(stream: Vec<Token>) -> Self {
            let root = Box::new(RootNode { children: Vec::new() });
            let idx: usize = 0;
            
            Self {
                stream,
                root,
                idx,
            }
        }

        pub fn construct(&mut self) {
            'construct: while self.idx <= self.stream.len() {
                // Set current token
                let current = &self.stream[self.idx];

            }   
        }

        fn match_token(&mut self, token: &Token) -> AstRes<Node> {
            match token.token_type {
                // Binary operators
                TokenType::Plus => {
                    // Get the left (previous) node
                    let last_node = match self.root.children.last() {
                        Some(last_node) => last_node,
                        None => {
                            todo!("Error handle no left node for binary op")
                        }
                    };

                    let ln = Box::new(*last_node);

                    
                    // Get the right (next) node
                    let next_token = &self.stream[&self.idx + 1];
                    let next_node = match self.match_token(next_token) {
                        // If valid node found
                        AstRes::Node(node) => node,

                        // If no valid node is found
                        AstRes::None => {
                            todo!("Error handle no right node for binary op")
                        },

                        // If EOF reached
                        AstRes::End => {
                            todo!("Error handle missing literal for binary operation")
                        }
                    };

                    let rn = Box::new(next_node);

                    // Create the binary expr struct
                    let expr = BinaryExpr { op: BinaryOp::Plus, ln, rn };

                    // Create binary expr node
                    let node = Node::BinaryExpr(expr);
                    self.root.push_node(node);

                    AstRes::Node(node)
                }


                // Number literals
                TokenType::NumberLit => {
                    // Check if number literal is a floating point num
                    if token.lexeme.contains(".") {
                        let parsed_num = match token.lexeme.parse::<f64>() {
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
                        
                        return AstRes::Node(node);
                    }

                    // Otherwise, make integer
                    let parsed_num = match token.lexeme.parse::<i32>() {
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

                    AstRes::Node(node)
                },

                // String literals
                TokenType::StringLit => {
                    let literal: LiteralExpr = LiteralExpr {
                        typ: LiteralType::String(String::from(&token.lexeme))
                    };

                    let node = Node::StringLitExpr(literal);
                    self.root.push_node(node);

                    AstRes::Node(node)
                },

                // End of file
                TokenType::EndFile => AstRes::End,

                _ => {
                    AstRes::None
                },
            }
        }
    }
}