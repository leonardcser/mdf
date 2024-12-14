use super::stmt::Stmt;
use super::tokens::Token;
use super::tokens::TokenKind::*;
use std::fmt::Debug;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxError {
    msg: String,
    pos: (usize, usize),
}

impl SyntaxError {
    pub fn new(msg: String, pos: (usize, usize)) -> Self {
        Self { msg, pos }
    }
}

#[derive(Clone, Debug)]
pub struct Parser<I: Iterator<Item = Token> + Clone + Debug> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token> + Clone + Debug> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<SyntaxError>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while let Some(token) = self.tokens.next() {
            match token.kind {
                Hashtag => {
                    // Try to parse a heading
                    match self.parse_heading(&token) {
                        Ok(heading) => statements.push(heading),
                        Err(error) => errors.push(error),
                    }
                }
                Word | Whitespace => {
                    if self.is_front_matter(&token) {
                        match self.parse_front_matter() {
                            Ok(font_matter) => statements.push(font_matter),
                            Err(error) => errors.push(error),
                        }
                    } else if self.is_code_block_start(&token) {
                        match self.parse_code_block(&token) {
                            Ok(code_block) => statements.push(code_block),
                            Err(error) => errors.push(error),
                        }
                    } else {
                        // Parse a paragraph
                        let paragraph = self.parse_paragraph(token);
                        statements.push(paragraph);
                    }
                }
                Newline => {
                    // Skip single newlines but detect blank lines
                    self.skip_blank_lines();
                }
                _ => {
                    // Handle unexpected tokens
                    errors.push(SyntaxError::new(
                        format!("Unexpected token: {:?}", token.kind),
                        token.pos,
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn is_front_matter(&self, first_token: &Token) -> bool {
        // TODO: Check if prev token is newline
        if first_token.text != "---" {
            return false;
        }
        // TODO: Check if line contains only spaces and ends with newline
        true
    }

    fn is_code_block_start(&self, first_token: &Token) -> bool {
        // TODO: Check if prev token is newline
        if !first_token.text.starts_with("```") {
            return false;
        }
        // TODO: Check if line contains only spaces and ends with newline
        true
    }

    fn parse_front_matter(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume_while(|t| t.kind == Whitespace);
        if let Some(token) = self.tokens.next() {
            assert_eq!(token.kind, Newline);
        }

        // Collect code block content
        let mut content = String::new();
        while let Some(token) = self.tokens.next() {
            if token.text == "---" {
                break;
            }
            content.push_str(&token.text);
        }

        Ok(Stmt::FrontMatter { content })
    }

    fn parse_code_block(&mut self, first_token: &Token) -> Result<Stmt, SyntaxError> {
        // Consume the code block delimiter
        let mut language = None;
        if first_token.text.starts_with("```") {
            if let Some(text) = first_token.text.strip_prefix("```") {
                if text.contains("`") {
                    return Err(SyntaxError::new(
                        "Invalid code block".to_string(),
                        first_token.pos,
                    ));
                }
                if !text.is_empty() {
                    language = Some(text.into());
                }
            }
        }

        self.consume_while(|t| t.kind == Whitespace);
        if let Some(token) = self.tokens.next() {
            assert_eq!(token.kind, Newline);
        }

        // Collect code block content
        let mut content = String::new();
        while let Some(token) = self.tokens.next() {
            if token.text == "```" {
                break;
            }
            content.push_str(&token.text);
        }

        Ok(Stmt::CodeBlock { language, content })
    }

    fn parse_heading(&mut self, first_token: &Token) -> Result<Stmt, SyntaxError> {
        let mut level = 1; // First `Hashtag` already counted
        while let Some(Hashtag) = self.tokens.peek().map(|t| &t.kind) {
            self.tokens.next(); // Consume the additional `Hashtag` tokens
            level += 1;
        }

        // Require a space after hashtags
        if let Some(Whitespace) = self.tokens.peek().map(|t| &t.kind) {
            self.tokens.next(); // Consume the space
        } else {
            return Err(SyntaxError::new(
                "Expected space after heading markers".to_string(),
                first_token.pos,
            ));
        }

        // Collect the rest of the line as the heading content
        let mut content = String::new();
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                Newline => break,
                _ => content.push_str(&self.tokens.next().unwrap().text),
            }
        }

        Ok(Stmt::Heading { level, content })
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

    fn consume_while<P>(&mut self, mut pred: P)
    where
        P: FnMut(&I::Item) -> bool,
    {
        while let Some(curr) = self.tokens.peek() {
            if pred(&curr) {
                self.tokens.next();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::lexer::Lexer;

    #[test]
    fn test_simple() {
        let input = "hello\n";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Paragraph {
                words: vec![Token::new(Word, "hello".into(), (1, 1)),]
            }]
        );
    }

    #[test]
    fn test_heading_invalid_1() {
        let input = "#heading";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Paragraph {
                words: vec![Token::new(Word, "#heading".into(), (1, 1)),]
            }]
        );
    }

    #[test]
    fn test_front_matter() {
        let input = "---\nvar: true\n---";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![Stmt::FrontMatter {
                content: "var: true\n".into()
            }]
        );
    }

    #[test]
    fn test_code_block_simple() {
        let input = "```\nlet x = 5;\nlet y = 10;\n```";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![Stmt::CodeBlock {
                language: None,
                content: "let x = 5;\nlet y = 10;\n".into()
            }]
        );
    }

    #[test]
    fn test_code_block_with_language() {
        let input = "```rust\nlet x = 5;\nlet y = 10;\n```";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![Stmt::CodeBlock {
                language: Some("rust".into()),
                content: "let x = 5;\nlet y = 10;\n".into()
            }]
        );
    }
    #[test]
    fn test_code_block_invalid() {
        let input = "````rust\nlet\n```";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        let mut parser = Parser::new(tokens.into_iter());
        if let Err(e) = parser.parse() {
            assert_eq!(
                e,
                vec![SyntaxError::new("Invalid code block".into(), (1, 1))]
            )
        } else {
            assert!(false);
        }
    }
}
