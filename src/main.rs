use crate::interpreter::{Interpreter, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};
use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod stdlib;

const HEX_BUILD: &str = "hexi 0.2.4";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        if !filename.ends_with(".hx") {
            eprintln!("[hexi::error] file must have .hx extension");
            std::process::exit(1);
        }

        run_file(filename);
    } else {
        run_repl();
    }
}

fn run_file(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("[hexi::error] reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    execute(&mut interpreter, &contents);
}

fn run_repl() {
    println!("{}", format!("{}. enter 'exit' or 'quit' to leave.", HEX_BUILD));
    let mut interpreter = Interpreter::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                println!("[hexi::error] error reading input: {}", e);
                continue;
            }
        }

        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("bye :3");
            break;
        }

        if input.is_empty() {
            continue;
        }
        
        execute(&mut interpreter, input);
    }
}

fn execute(interpreter: &mut Interpreter, code: &str) {
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let exprs = match parser.parse() {
        Ok(e) => e,
        Err(e) => {
            println!("parser error: {}", e);
            return;
        }
    };

    for expr in exprs {
        match interpreter.evaluate(&expr) {
            Err(e) => {
                println!("runtime error: {}", e);
                break;
            },
            Ok(result) => {
                if result != Value::Nil {
                    println!("{}", result);
                }
            },
        }
    }
}