# Darcy2 :D
Second attempt at my language implementation in Rust

## Updates
- This is a rewrite of former darcylang, started on Mar 5 2024
- AST generation does not work but the infrastructure is there (I need to implement it in main.rs)
- Also fixed an embarassing mistake where I iterated through every char looking for newlines isntead of just .split("\n") O_O

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

## Parser TODO
- [x] Complete AST infrastructure and nodes
- [ ] Generate and push nodes from AST