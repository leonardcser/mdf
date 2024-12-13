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

    // pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<SyntaxError>> {
    //     let mut statements = vec![];
    //     let mut errors = vec![];
    //     while !self.at_end() {
    //         let stmt = self.declaration();
    //         if let Ok(val) = stmt {
    //             statements.push(val);
    //         } else {
    //             if let Err(e) = stmt {
    //                 println!("[line {}] {}", e.line, e.msg.to_string());
    //                 errors.push(e);
    //             }
    //         }
    //     }
    //     if errors.is_empty() {
    //         return Ok(statements);
    //     }
    //     Err(errors)
    // }
}
