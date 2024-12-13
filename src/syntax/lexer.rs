use super::tokens::TokenKind::*;
use super::tokens::*;
use std::iter::Peekable;
use std::str::CharIndices;

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
                self.col = 1;
                self.line += 1;
                self.yield_token(Newline)
            }
            ' ' => self.yield_token(Whitespace),
            '#' if self.is_hashtag() => self.yield_token(Hashtag),
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
            line: 1,
            col: 1,
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
        let start = match self.current {
            Some((idx, _)) => idx,
            None => return "".into(),
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
        self.source[start..=end].into()
    }

    fn consume_char(&mut self) {
        self.current = self.chars.next();
        self.col += 1;
    }

    fn yield_token(&self, kind: TokenKind) -> Token {
        let start = self.current.unwrap().0;
        Token::new(kind, self.source[start..start + 1].into())
    }

    // fn peek_chars(&self, count: usize) -> Vec<Option<(usize, char)>> {
    //     let mut peek_chars = self.chars.clone();
    //     (0..count).map(|_| peek_chars.next()).collect()
    // }
    //
    // fn consume_while<P>(&mut self, mut pred: P)
    // where
    //     P: FnMut(&(usize, char)) -> bool,
    // {
    //     while let Some(&curr) = self.chars.peek() {
    //         if pred(&curr) {
    //             self.consume_char();
    //         } else {
    //             break;
    //         }
    //     }
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
                Token::new(Word, "hello".into()),
                Token::new(Newline, "\n".into()),
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
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "1".into()),
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
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "2".into()),
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
                Token::new(Whitespace, " ".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into()),
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
                Token::new(Newline, "\n".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into()),
            ]
        );
    }
    #[test]
    fn test_tokenize_heading_invalid_1() {
        let input = "#Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens, vec![Token::new(Word, "#Heading".into()),]);
    }

    #[test]
    fn test_tokenize_heading_invalid_2() {
        let input = "####### Heading";
        let lexer = Lexer::new(input);

        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into())
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
                Token::new(Whitespace, " ".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Hashtag, "#".into()),
                Token::new(Whitespace, " ".into()),
                Token::new(Word, "Heading".into())
            ]
        );
    }
}
