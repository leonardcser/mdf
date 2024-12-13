use super::stmt::Stmt;
use super::tokens::Token;
use super::tokens::TokenKind::*;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<SyntaxError>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while let Some(token) = self.tokens.next() {
            match token.kind {
                Hashtag => {
                    // Try to parse a heading
                    match self.parse_heading(token) {
                        Ok(heading) => statements.push(heading),
                        Err(error) => errors.push(error),
                    }
                }
                Word | Whitespace => {
                    // Parse a paragraph
                    let paragraph = self.parse_paragraph(token);
                    statements.push(paragraph);
                }
                Newline => {
                    // Skip single newlines but detect blank lines
                    self.skip_blank_lines();
                }
                _ => {
                    // Handle unexpected tokens
                    errors.push(SyntaxError {
                        msg: format!("Unexpected token: {:?}", token.kind),
                        line: 0, // Add proper line tracking if needed
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn parse_heading(&mut self, first_token: Token) -> Result<Stmt, SyntaxError> {
        let mut level = 1; // First `Hashtag` already counted
        while let Some(Hashtag) = self.tokens.peek().map(|t| &t.kind) {
            self.tokens.next(); // Consume the additional `Hashtag` tokens
            level += 1;
        }

        // Require a space after hashtags
        if let Some(Whitespace) = self.tokens.peek().map(|t| &t.kind) {
            self.tokens.next(); // Consume the space
        } else {
            return Err(SyntaxError {
                msg: "Expected space after heading markers".to_string(),
                line: 0, // Add proper line tracking
            });
        }

        // Collect the rest of the line as the heading content
        let mut text = String::new();
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                Newline => break,
                _ => text.push_str(&self.tokens.next().unwrap().text),
            }
        }

        Ok(Stmt::Heading { level, text })
    }

    fn parse_paragraph(&mut self, first_token: Token) -> Stmt {
        let mut words = vec![first_token];
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                Newline => {
                    // Check for blank lines to end the paragraph
                    self.tokens.next(); // Consume the newline
                    if let Some(Newline) = self.tokens.peek().map(|t| &t.kind) {
                        break; // End of paragraph
                    }
                }
                Whitespace => {
                    self.tokens.next();
                }
                _ => words.push(self.tokens.next().unwrap()), // Add word/whitespace to the paragraph
            }
        }

        Stmt::Paragraph { words }
    }

    fn skip_blank_lines(&mut self) {
        while let Some(Newline) = self.tokens.peek().map(|t| &t.kind) {
            self.tokens.next(); // Consume consecutive newlines
        }
    }
}
