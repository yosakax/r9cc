use anyhow;
use std::env;
use thiserror;

#[derive(Debug)]
enum Token {
    Integer(i64),
    Operator(String),
}
#[derive(thiserror::Error, Debug)]
#[error("token error")]
pub struct TokenError;

#[derive(thiserror::Error, Debug)]
#[error("unknown token: {token}")]
pub struct UnknownTokenError {
    token: String,
}

#[derive(thiserror::Error, Debug)]
#[error("unknown operator: {operator}")]
pub struct UnknownOperatorError {
    operator: String,
}

fn is_digit(c: &char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn tokenize(input_str: &String) -> anyhow::Result<Vec<Token>> {
    let mut result = vec![];
    let input_vec: Vec<char> = input_str.to_owned().chars().collect();

    let mut s = String::new();
    let last_index = input_str.len();
    for idx in 0..last_index {
        let c = input_vec[idx];
        let next_c = if idx == last_index - 1 {
            '\n'
        } else {
            input_vec[idx + 1]
        };

        match c {
            '+' => result.push(Token::Operator(String::from("+"))),
            '-' => result.push(Token::Operator(String::from("-"))),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if is_digit(&next_c) {
                    s.push(c);
                } else {
                    s.push(c);
                    if let Ok(val) = s.parse::<i64>() {
                        result.push(Token::Integer(val));
                        s.clear();
                    } else {
                        eprintln!("Tokenize error parse::<i64> {}", s);
                    }
                }
            }
            _ => Err(UnknownTokenError {
                token: c.to_string(),
            })
            .unwrap(),
        }
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("Invalid arguments number.");
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // println!("  mov rax, {}", argv[1]);
    let tokens = tokenize(&argv[1]).expect("failed to tokenize");
    let first_token = match tokens[0] {
        Token::Integer(n) => n,
        _ => return Err(TokenError)?,
    };
    println!("  mov rax, {}", first_token);
    for token in tokens[1..].iter() {
        match token {
            Token::Integer(n) => println!("{}", n),
            Token::Operator(s) => match s.as_str() {
                "+" => {
                    print!("  add rax, ")
                }
                "-" => {
                    print!("  sub rax, ")
                }
                _ => {
                    return Err(UnknownOperatorError {
                        operator: s.to_string(),
                    })?
                }
            },
        }
    }
    // eprintln!("{:?}", tokens);

    println!("  ret");
    Ok(())
}
