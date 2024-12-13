use super::tokens::TokenKind::*;
use super::tokens::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Heading,
    Paragraph { words: Vec<Token> },
}
