use super::tokens::TokenKind::*;
use super::tokens::*;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Clone, Debug)]
pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
    current: Option<(usize, char)>,
    line: usize,
    col: usize,
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_char();
        let (_start, ch) = self.current?;
        let token = match ch {
            '\n' => {
                let t = self.yield_token(Newline);
                self.col = 0;
                self.line += 1;
                t
            }
            ' ' => self.yield_token(Whitespace),
            '#' if self.is_hashtag() => self.yield_token(Hashtag),
            ch if self.is_word(ch) => {
                let pos = (self.line, self.col);
                Token::new(Word, self.consume_word(), pos)
            }
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
            line: 1,
            col: 0,
        }
    }

    fn is_hashtag(&mut self) -> bool {
        if let Some(&(_, next_ch)) = self.chars.peek() {
            if next_ch == '#' || next_ch.is_whitespace() {
                return true;
            }
        }
        return false;
    }

    fn is_word(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch.is_ascii_punctuation()
    }

    fn consume_word(&mut self) -> String {
        // Store the start index of the word
        let (start, ch) = match self.current {
            Some((idx, ch)) => (idx, ch),
            None => return "".into(),
        };

        // Keep track of the last index of the word
        let mut end = start + ch.len_utf8() - 1;

        // Continue consuming alphanumeric characters
        while let Some(&(next_idx, next_ch)) = self.chars.peek() {
            if self.is_word(next_ch) {
                let len = next_ch.len_utf8();
                end = next_idx + len - 1;
                // Actually consume the character
                let tmp_col = self.col;
                for _ in 0..len {
                    self.consume_char();
                }
                self.col = tmp_col + 1;
            } else {
                break;
            }
        }

        // Return the slice representing the full word
        dbg!(start, end, &self.source[start..=end]);
        self.source[start..=end].into()
    }

    fn consume_char(&mut self) {
        self.current = self.chars.next();
        self.col += 1;
    }

    fn yield_token(&self, kind: TokenKind) -> Token {
        let (start, ch) = self.current.unwrap();
        Token::new(
            kind,
            self.source[start..start + ch.len_utf8()].into(),
            (self.line, self.col),
        )
    }

    // fn peek_chars(&self, count: usize) -> Vec<Option<(usize, char)>> {
    //     let mut peek_chars = self.chars.clone();
    //     (0..count).map(|_| peek_chars.next()).collect()
    // }
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
                Token::new(Word, "hello".into(), (1, 1)),
                Token::new(Newline, "\n".into(), (1, 6)),
            ]
        );
    }

    #[test]
    fn test_word_1() {
        let input = "```rust\n";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Word, "```rust".into(), (1, 1)),
                Token::new(Newline, "\n".into(), (1, 8)),
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
            vec![
                Token::new(Hashtag, "#".into(), (1, 1)),
                Token::new(Whitespace, " ".into(), (1, 2)),
                Token::new(Word, "Heading".into(), (1, 3)),
                Token::new(Whitespace, " ".into(), (1, 10)),
                Token::new(Word, "1".into(), (1, 11)),
            ]
        );
    }

    #[test]
    fn test_tokenize_heading_2() {
        let input = "## Heading 2";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Hashtag, "#".into(), (1, 1)),
                Token::new(Hashtag, "#".into(), (1, 2)),
                Token::new(Whitespace, " ".into(), (1, 3)),
                Token::new(Word, "Heading".into(), (1, 4)),
                Token::new(Whitespace, " ".into(), (1, 11)),
                Token::new(Word, "2".into(), (1, 12)),
            ]
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
                Token::new(Whitespace, " ".into(), (1, 1)),
                Token::new(Whitespace, " ".into(), (1, 2)),
                Token::new(Hashtag, "#".into(), (1, 3)),
                Token::new(Hashtag, "#".into(), (1, 4)),
                Token::new(Whitespace, " ".into(), (1, 5)),
                Token::new(Word, "Heading".into(), (1, 6)),
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
                Token::new(Newline, "\n".into(), (1, 1)),
                Token::new(Hashtag, "#".into(), (2, 1)),
                Token::new(Hashtag, "#".into(), (2, 2)),
                Token::new(Whitespace, " ".into(), (2, 3)),
                Token::new(Word, "Heading".into(), (2, 4)),
            ]
        );
    }
    #[test]
    fn test_tokenize_heading_invalid_1() {
        let input = "#Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens, vec![Token::new(Word, "#Heading".into(), (1, 1)),]);
    }

    #[test]
    fn test_tokenize_heading_invalid_2() {
        let input = "####### Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Hashtag, "#".into(), (1, 1)),
                Token::new(Hashtag, "#".into(), (1, 2)),
                Token::new(Hashtag, "#".into(), (1, 3)),
                Token::new(Hashtag, "#".into(), (1, 4)),
                Token::new(Hashtag, "#".into(), (1, 5)),
                Token::new(Hashtag, "#".into(), (1, 6)),
                Token::new(Hashtag, "#".into(), (1, 7)),
                Token::new(Whitespace, " ".into(), (1, 8)),
                Token::new(Word, "Heading".into(), (1, 9))
            ]
        );
    }

    #[test]
    fn test_tokenize_heading_invalid_3() {
        let input = "   ## Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Whitespace, " ".into(), (1, 1)),
                Token::new(Whitespace, " ".into(), (1, 2)),
                Token::new(Whitespace, " ".into(), (1, 3)),
                Token::new(Hashtag, "#".into(), (1, 4)),
                Token::new(Hashtag, "#".into(), (1, 5)),
                Token::new(Whitespace, " ".into(), (1, 6)),
                Token::new(Word, "Heading".into(), (1, 7))
            ]
        );
    }
}
