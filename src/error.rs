use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Errors {
    #[error("Unexpected token at line: {0} column: {1}")]
    UnexpectedToken(i64, i64),
    #[error("Expect more tokens, the last parsed token is at line: {0} column: {1}")]
    ExpectMoreToken(i64, i64),
    #[error("{0}")]
    RuntimeException(String),
    #[error("Unknown error")]
    Unknown,
}
