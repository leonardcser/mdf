use crate::syntax::tokens::TokenKind::*;
use crate::syntax::tokens::*;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
    current: Option<(usize, char)>,
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_char();
        let (_start, ch) = self.current?;
        let token = match ch {
            '\n' => self.yield_token(Newline),
            ' ' => self.yield_token(Whitespace),
            _ if self.is_heading() => self.consume_heading(),
            ch if self.is_word(ch) => Token::new(Word, self.consume_word()),
            _ => self.yield_token(Illegal),
        };
        Some(token)
    }
}

impl<'s> Lexer<'s> {
    pub fn new(string: &'s str) -> Self {
        Self {
            source: string,
            chars: string.char_indices().peekable(),
            current: None,
        }
    }

    fn consume_char(&mut self) {
        self.current = self.chars.next();
    }

    fn yield_token(&self, kind: TokenKind) -> Token {
        let start = self.current.unwrap().0;
        Token::new(kind, self.source[start..start + 1].to_string())
    }

    fn peek_chars(&self, count: usize) -> Vec<Option<(usize, char)>> {
        let mut peek_chars = self.chars.clone();
        (0..count).map(|_| peek_chars.next()).collect()
    }

    fn is_heading(&self) -> bool {
        let (_, ch) = match self.current {
            Some((idx, ch)) => (idx, ch),
            None => return false,
        };

        // If not at the start, check if preceded by a newline
        if ch != '#' {
            return false;
        }

        let mut level = 1;
        for peek in self.peek_chars(6).iter() {
            match peek {
                Some((_, '#')) => {
                    level += 1;
                    if level >= 6 {
                        return false;
                    }
                }
                Some((_, ch)) if ch.is_whitespace() => return true,
                _ => break,
            }
        }

        false
    }

    fn consume_while<P>(&mut self, mut pred: P)
    where
        P: FnMut(&(usize, char)) -> bool,
    {
        while let Some(&curr) = self.chars.peek() {
            if pred(&curr) {
                self.consume_char();
            } else {
                self.consume_char();
                break;
            }
        }
    }

    fn consume_heading(&mut self) -> Token {
        let mut level = 1;
        self.consume_while(|(_, c)| {
            if *c == '#' {
                level += 1;
                assert_ne!(level, 7);
                return true;
            }
            false
        });
        self.consume_while(|(_, c)| c.is_whitespace());

        let start = match self.current {
            Some((idx, _)) => idx,
            None => return Token::new(Illegal, "".to_string()),
        };
        self.consume_while(|(_, c)| *c != '\n');

        // Create a heading token with its level and content
        Token::new(
            Heading(HeadingToken { level }),
            self.source[start..=self.current.unwrap().0]
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
                .to_string(),
        )
    }

    fn is_word(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch.is_ascii_punctuation()
    }

    fn consume_word(&mut self) -> String {
        // Store the start index of the word
        let start = match self.current {
            Some((idx, _)) => idx,
            None => return "".to_string(),
        };

        // Keep track of the last index of the word
        let mut end = start;

        // Continue consuming alphanumeric characters
        while let Some(&(next_idx, next_ch)) = self.chars.peek() {
            if self.is_word(next_ch) {
                end = next_idx;
                // Actually consume the character
                self.consume_char();
            } else {
                break;
            }
        }

        // Return the slice representing the full word
        self.source[start..=end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "hello\n";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Word, "hello".to_string()),
                Token::new(Newline, "\n".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_heading_1() {
        let input = "# Heading 1";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![Token::new(
                Heading(HeadingToken { level: 1 }),
                "Heading 1".to_string()
            ),]
        );
    }

    #[test]
    fn test_tokenize_heading_2() {
        let input = "## Heading 2";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![Token::new(
                Heading(HeadingToken { level: 2 }),
                "Heading 2".to_string()
            ),]
        );
    }

    #[test]
    fn test_tokenize_heading_valid_1() {
        let input = "  ## Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Whitespace, " ".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Heading(HeadingToken { level: 2 }), "Heading".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_heading_multiline() {
        let input = "\n## Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Newline, "\n".to_string()),
                Token::new(Heading(HeadingToken { level: 2 }), "Heading".to_string()),
            ]
        );
    }
    #[test]
    fn test_tokenize_heading_invalid_1() {
        let input = "#Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens, vec![Token::new(Word, "#Heading".to_string()),]);
    }

    #[test]
    fn test_tokenize_heading_invalid_2() {
        let input = "####### Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Word, "#######".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Word, "Heading".to_string())
            ]
        );
    }

    #[ignore = "Not implemented yet"]
    #[test]
    fn test_tokenize_heading_invalid_3() {
        let input = "   ## Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Whitespace, " ".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Word, "##".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Word, "Heading".to_string())
            ]
        );
    }
}
