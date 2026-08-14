# rust-lisp

A simple LISP interpreter, written in Rust as a learning exercise.

Functionality is currently limited to: reading in an input string representing an expression, tokenizing it, parsing to AST and eval of simple arithmetic (+ - * /).

## Quickstart

Simply edit `main()` and change `input` to your desired LISP expression.

Run via `cargo run` and the input, tokenized form, AST expression and evaluation will be displayed. Or it will panic :)
