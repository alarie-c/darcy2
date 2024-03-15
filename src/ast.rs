pub mod node {
    use super::ast::NodeKey;


    #[derive(Debug, Clone, PartialEq)]
    pub enum Node {
        BinaryExpr(BinaryExpr),

        // Literals
        StringLitExpr(LiteralExpr),
        NumberLitExpr(LiteralExpr),
        IntegerLitExpr(LiteralExpr),

        // Other
        RootNode,
        EndNode,
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
        pub ln: NodeKey,
        pub rn: NodeKey,
    }   

    #[derive(Debug, Clone, PartialEq)]
    pub struct LiteralExpr {
        pub typ: LiteralType,
    }
}

pub mod ast {
    use std::{iter::Peekable, vec::IntoIter};

    use slotmap::{new_key_type, SlotMap};
    use crate::token::token::{TokenType, Token};
    use super::node::{LiteralExpr, LiteralType, Node};

    // Define custom result type
    enum AstRes<Node> {
        Match(Node),
        None,
        End,
    }

    // Define slotmap key type
    new_key_type! {
        pub struct NodeKey;
    }

    pub struct Ast {
        pub stream: Peekable<IntoIter<Token>>,
        pub tree: SlotMap<NodeKey, Node>,
        pub root: NodeKey,
        pub current: Token,
        pub keys: Vec<NodeKey>,
    }

    impl Ast {
        pub fn new(tokens: Vec<Token>) -> Self {
            let mut tree: SlotMap<NodeKey, Node> = SlotMap::with_key();
            let keys = Vec::<NodeKey>::new();

            // Construct iterator from tokens
            let mut stream = tokens.into_iter().peekable();
            let current = match stream.next() {
                Some(current) => current,
                None => {
                    eprintln!("Recieved an empty token stream!");
                    std::process::exit(1);
                }
            };

            // Create root node and construct
            let root = tree.insert(Node::RootNode);
            Self { stream, tree, root, current, keys }
        }

        // Takes the stream of incoming tokens and constructs an
        // abstract syntax tree based on it
        pub fn parse(&mut self) {
            'parse: loop {
                let node = match self.match_token() {
                    AstRes::Match(n) => {
                        self.push_node(n);
                    },
                    AstRes::None => {
                        
                    },
                    AstRes::End => {
                        self.push_node(Node::EndNode);
                        break 'parse;
                    },
                };

                // Advance to next token if possible
                if self.advance() {
                    continue 'parse;
                } else {
                    break 'parse;
                }
            }
        }

        fn push_node(&mut self, node: Node) {
            // Push node to tree and get it's unique key
            let key = self.tree.insert(node);

            // Push that key to the Keys helper vec
            self.keys.push(key);
        }

        fn match_token(&mut self) -> AstRes<Node> {
            match self.current.token_type {
                // Number literals
                TokenType::NumberLit => {
                    // Determine if literal is float or integer
                    // TODO: Impliment type hinting
                    
                    if self.current.lexeme.contains(".") {
                        // If float
                        let parsed_num = match self.current.lexeme.parse::<f64>() {
                            Ok(parsed_num) => parsed_num,
                            Err(_) => {
                                todo!("Error handle failure to convert string to float");
                            }
                        };

                        let literal_expr = LiteralExpr { typ: LiteralType::Number(parsed_num) };
                        return AstRes::Match(Node::NumberLitExpr(literal_expr));
                    } else {
                        // If int
                        let parsed_num = match self.current.lexeme.parse::<i32>() {
                            Ok(parsed_num) => parsed_num,
                            Err(_) => {
                                todo!("Error handle failure to convert string to float");
                            }
                        };

                        let literal_expr = LiteralExpr { typ: LiteralType::Integer(parsed_num) };
                        return AstRes::Match(Node::NumberLitExpr(literal_expr));
                    }
                },

                // String literal
                TokenType::StringLit => {
                    let literal_expr = LiteralExpr { typ: LiteralType::String(self.current.lexeme.to_string()) };
                    return AstRes::Match(Node::StringLitExpr(literal_expr));
                }

                // End of file token
                TokenType::EndFile => return AstRes::End,

                _ => return AstRes::None,
            }
        } 

        fn advance(&mut self) -> bool {
            if let Some(t) = self.stream.next() {
                self.current = t;
                true
            } else {
                false
            }
        }
    }
}