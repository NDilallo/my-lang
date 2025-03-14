use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Clone)]
enum TokenType {
    _return,
    int_literal,
    semimicolon
}

#[derive(Clone)]
struct Token{
    token_type: Option<TokenType>,
    value: Option<String>
}

fn tokenize(input: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut curr_token: Token = Token { token_type: None, value: None };

    for c in input.chars() {
        match c {
            ' ' => {
                tokens.push(curr_token.clone());
                curr_token = Token { token_type: None, value: None };
            }
            ';' => {
                tokens.push(curr_token.clone());
                tokens.push(Token { token_type: Some(TokenType::semimicolon), value: Some(";".to_string()) })
            }
            '0'..'9' => {
                if curr_token.token_type.is_none() {
                    curr_token.token_type = Some(TokenType::int_literal);
                }
                if let Some(value) = curr_token.value.as_mut() {
                    value.push(c);
                } else {
                    curr_token.value = Some(c.to_string());
                }
            }
            _ => {
                if curr_token.token_type.is_none() {
                    curr_token.token_type = Some(TokenType::_return);
                }
                if let Some(value) = curr_token.value.as_mut() {
                    value.push(c);
                } else {
                    curr_token.value = Some(c.to_string());
                }
                
            }
        }
    }
    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: {:?}", args);

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let contents = match read_file(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let tokens = tokenize(&contents);
    //print tokens vector
    for token in tokens {
        match token.token_type {
            Some(TokenType::int_literal) => println!("int_literal: {}", token.value.unwrap()),
            Some(TokenType::semimicolon) => println!("semimicolon: {}", token.value.unwrap()),
            Some(TokenType::_return) => println!("_return: {}", token.value.unwrap()),
            None => println!("None")
        }
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}