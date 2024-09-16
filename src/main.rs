extern crate r9cc;
use anyhow;
use r9cc::{Token, TokenError, TokenKind, UnknownOperatorError, UnknownTokenError};
use std::env;

fn is_digit(c: &char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn tokenize(mut input_str: String) -> anyhow::Result<Vec<Token>> {
    let mut result = vec![];

    let mut s = String::new();
    while let Some(c) = input_str.chars().nth(0) {
        if c.is_whitespace() {
            input_str = input_str.split_off(1);
            continue;
        }
        match c {
            '+' => {
                if !s.is_empty() {
                    if let Ok(val) = s.parse::<i64>() {
                        let token =
                            Token::new(TokenKind::Integer, Option::None, Some(val), Option::None);
                        result.push(token);
                        s.clear();
                    } else {
                        eprintln!("Tokenize error parse::<i64> {}", s);
                    }
                }
                let token = Token::new(
                    TokenKind::Operator,
                    Option::None,
                    Option::None,
                    Some(String::from("+")),
                );
                result.push(token);
            }
            '-' => {
                if !s.is_empty() {
                    if let Ok(val) = s.parse::<i64>() {
                        let token =
                            Token::new(TokenKind::Integer, Option::None, Some(val), Option::None);
                        result.push(token);
                        s.clear();
                    } else {
                        eprintln!("Tokenize error parse::<i64> {}", s);
                    }
                }
                let token = Token::new(
                    TokenKind::Operator,
                    Option::None,
                    Option::None,
                    Some(String::from("-")),
                );
                result.push(token);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                s.push(c);
            }
            _ => Err(UnknownTokenError {
                token: c.to_string(),
            })
            .unwrap(),
        }
        input_str = input_str.split_off(1); // *p++;
    }
    if !s.is_empty() {
        if let Ok(val) = s.parse::<i64>() {
            let token = Token::new(TokenKind::Integer, Option::None, Some(val), Option::None);
            result.push(token);
            s.clear();
        } else {
            eprintln!("Tokenize error parse::<i64> {}", s);
        }
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let mut argv = env::args();
    // eprintln!("input:\n{:?}", argv.nth(1).unwrap());

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // println!("  mov rax, {}", argv[1]);
    let tokens = tokenize(argv.nth(1).unwrap()).expect("failed to tokenize");
    eprintln!("{:?}", tokens);
    let first_token = match tokens[0].kind {
        TokenKind::Integer => tokens[0].val.unwrap(),
        _ => return Err(TokenError)?,
    };
    println!("  mov rax, {}", first_token);
    for token in tokens[1..].iter() {
        match token.kind {
            TokenKind::Integer => println!("{}", token.val.unwrap()),
            TokenKind::Operator => match token.str.clone().unwrap().as_str() {
                "+" => {
                    print!("  add rax, ")
                }
                "-" => {
                    print!("  sub rax, ")
                }
                _ => {
                    return Err(UnknownOperatorError {
                        operator: token.str.to_owned().unwrap().to_string(),
                    })?
                }
            },
            _ => {}
        }
    }
    // eprintln!("{:?}", tokens);

    println!("  ret");
    Ok(())
}
