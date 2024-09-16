use thiserror;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TokenKind {
    Integer,
    Operator,
    Eof,
}

#[derive(thiserror::Error, Debug)]
#[error("token error")]
pub struct TokenError;

#[derive(thiserror::Error, Debug)]
#[error("unknown token: {token}")]
pub struct UnknownTokenError {
    pub token: String,
}

#[derive(thiserror::Error, Debug)]
#[error("unknown operator: {operator}")]
pub struct UnknownOperatorError {
    pub operator: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Token {
    pub kind: TokenKind,
    // pub next: Option<Box<Token>>,
    pub next: Option<Box<Token>>,
    pub val: Option<i64>,
    pub str: Option<String>,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        next: Option<Box<Token>>,
        val: Option<i64>,
        str: Option<String>,
    ) -> Self {
        Token {
            kind,
            next,
            val,
            str,
        }
    }
}
