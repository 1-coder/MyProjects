mod scanner;

use crate::scanner::*;

use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    } else {
        match run_promt() {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        };
    }

    println!("Exiting jlox");
}

fn run_file(path: &str) -> Result<(), String> {
    // let mut contents =  String::new() ;
    match fs::read_to_string(path) {
        Ok(contents) => match run(&contents) {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        },
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}

fn run_promt() -> Result<(), String> {
    loop {
        print!(">");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(_) => {}
        }
        let mut input = String::new();
        let mut handle = io::stdin().lock();
        match handle.read_line(&mut input) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }
        if input.is_empty() || input == "\n" {
            break;
        }

        println!("ECHO: {}", input); 
        match run(&input){
            Ok(_) => {},
            Err(e) => println!("{e}")
        }
    }
    Ok(())
}

fn run(contents: &str) -> Result<(), String> {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
