<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
mod scanner;
mod error;
mod token;

use std::env;

use std::io::{self, BufRead, Write};
use std::fs;
use std::error::Error;

use crate::scanner::*;
use crate::error::*;
use crate::token::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    //Invalid Arguments
    if args.len() > 2{
        println!("Usage: rust-lox [script]");
        std::process::exit(64);
    //One argument - file to run
    } else if args.len()==2 {
        run_file(&args[1][..]).expect("Failed to run file");
    }
    //No arguments - REPL
    else{
        run_prompt().expect("Error running prompt");
    }
}


fn run_file(file: &str) -> Result<(), Box<dyn Error>>{
    
    let input = fs::read_to_string(file)?;
    match run(&input[..]) {
        Ok(_) => {},
        Err(e) => e.report()
    }
    Ok(())
}


fn run_prompt() -> io::Result<()>{
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        if input.trim().eq("exit") {break}
        if input.trim().is_empty() {continue}
        match run(input.trim()) {
            Ok(_) => {},
            Err(e) => e.report()
        };


    }
    Ok(())
}

fn run(source: &str) -> Result<(), InterpreterError> {
    
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens()?;

    for token in tokens.iter() {
        println!("{:?}", token);
    }
    Ok(())
>>>>>>> 2e8a1fe (Up to "Longer Lexemes" in the Scanning chapter)
}
