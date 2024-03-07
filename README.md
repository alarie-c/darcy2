# Darcy2 :D
Second attempt at my language implementation in Rust

## Updates
- This is a rewrite of former darcylang, started on Mar 5 2024
- AST generation does NOT work currently because I need to figure out lifetimes first
    - Binary nodes and nodes that have child nodes need to take ownership of formerly created and stored nodes

## Project Milestones
- [x] Complete lexer (symbols, keywords, and literals)
- [ ] Complete AST generation (binary, unary, keyword, and literals)
- [ ] Complete AST evaluator
- [ ] Create error handing infrastructure
- [ ] Create scope and environment infrastructure
- [ ] Implement scope and error handling to lexer/ast/eval

## Main TODO
- [x] Read file from env args
- [x] Generate `Vec<String>` of file lines

## Lexer TODO
- [x] Create lexer struct
- [x] Implement helper functions
- [ ] Complete `scan()` method