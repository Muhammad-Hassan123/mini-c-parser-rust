# Mini-C parser in Rust

The front end of a compiler for a small C-like language. It reads a source file, turns it into tokens and builds an abstract syntax tree.

## What's here

- **Three lexers**, written to compare approaches: hand-written (`src/lexer_manual.rs`), regex-based (`src/lexer_regex.rs`) and rules-based (`src/rules.rs`)
- **A recursive-descent parser** for statements and expressions (`src/parser.rs`), with typed parse errors
- **The grammar** in BNF (`grammar/mini_C.bnf`)
- **The AST** (`src/ast.rs`), exported to JSON

It stops at the syntax tree. There's no type checking or code generation yet.

## Run it

```
cargo run -- test_statements.c
```

Tokens print to the console and are written to `regex_tokens.txt` and `manual_tokens.txt`. The tree goes to `ast_output.json`.
