use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use codegen::CodeGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: toylang <script.toy>");
        process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });

    // Lexing
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer Error: {}", err);
            process::exit(1);
        }
    };

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parser Error: {}", err);
            process::exit(1);
        }
    };

    // Code Generation (Cranelift)
    let mut codegen = match CodeGenerator::new() {
        Ok(cg) => cg,
        Err(err) => {
            eprintln!("Codegen Initialization Error: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = codegen.generate(&ast) {
        eprintln!("Codegen Error: {}", err);
        process::exit(1);
    }
}
