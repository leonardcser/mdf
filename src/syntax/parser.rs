use super::tokens::Token;
use std::iter::Peekable;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    msg: String,
    line: usize,
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    processed_tokens: Vec<Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
            processed_tokens: vec![],
        }
    }
}
