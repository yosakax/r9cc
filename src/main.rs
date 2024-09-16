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
            '+' => {
                let token = Token::new(
                    TokenKind::Operator,
                    Option::None,
                    Option::None,
                    Some(String::from("+")),
                );
                result.push(token);
            }
            '-' => {
                let token = Token::new(
                    TokenKind::Operator,
                    Option::None,
                    Option::None,
                    Some(String::from("-")),
                );
                result.push(token);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if is_digit(&next_c) {
                    s.push(c);
                } else {
                    s.push(c);
                    if let Ok(val) = s.parse::<i64>() {
                        let token =
                            Token::new(TokenKind::Integer, Option::None, Some(val), Option::None);
                        result.push(token);
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

/// 次のTokenが期待している記号のときにはTokenを１つ読み勧めて
/// 真を返す。それ以外の場合は偽
fn consume(token: &mut Token, op: String) -> bool {
    if token.kind == TokenKind::Operator || token.to_owned().str.unwrap() != op {
        return false;
    }
    // token = token.next.unwrap().into();
    if let Some(next_token) = token.next.take() {
        *token = *next_token;
    }

    return true;
}

fn expect(token: &mut Token, op: String) {
    if token.kind != TokenKind::Operator || token.to_owned().str.unwrap() != op {
        panic!("not : {}", op);
    }
    // token = token.to_owned().next.unwrap();
    if let Some(next_token) = token.next.take() {
        *token = *next_token;
    }
}

fn expect_number(token: &mut Token) -> i64 {
    if token.kind != TokenKind::Integer {
        panic!("not a number!");
    }
    let val = token.val.unwrap();
    if let Some(next_token) = token.next.take() {
        *token = *next_token;
    }
    val
}

fn at_eof(token: &Token) -> bool {
    return token.kind == TokenKind::Eof;
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = env::args().collect();
    let mut input_vec = vec![];
    for i in 1..argv.len() {
        input_vec.push(argv[i].to_owned());
    }
    let input = input_vec.join("");

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // println!("  mov rax, {}", argv[1]);
    let tokens = tokenize(&input).expect("failed to tokenize");
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
