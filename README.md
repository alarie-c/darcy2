# Darcy2 :D
Second attempt at my language implementation in Rust

## Updates
- This is a rewrite of former darcylang, started on Mar 5 2024

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