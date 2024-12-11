use std::io;

/// A simple Token enum to represent different types of tokens in the markdown
#[derive(Debug, PartialEq)]
pub enum Token {
    Word(WordToken),   // Inline word tokens (like normal text or inline code)
    Line(LineToken),   // Header line tokens
    Block(BlockToken), // Block tokens (code blocks, front matter)
}

/// Struct to represent inline word tokens
#[derive(Debug, PartialEq)]
pub struct WordToken {
    pub text: String,
    pub kind: WordKind,
}

/// Enum to represent the kind of word token (e.g., inline code, bold, or normal)
#[derive(Debug, PartialEq)]
pub enum WordKind {
    Normal,
    InlineCode,
    Bold,
    Italic,
}

/// Struct to represent line tokens (headers)
#[derive(Debug, PartialEq)]
pub struct LineToken {
    pub level: usize, // Number of `#` symbols in a header
    pub content: String,
}

/// Struct to represent block tokens (code blocks, front matter, etc.)
#[derive(Debug, PartialEq)]
pub struct BlockToken {
    pub kind: BlockKind,
    pub content: String,
}

/// Enum to represent the kind of block token (e.g., code block, front matter)
#[derive(Debug, PartialEq)]
pub enum BlockKind {
    CodeBlock,
    FrontMatter,
}

impl Token {
    pub fn is_word(&self) -> bool {
        matches!(self, Token::Word(_))
    }

    pub fn is_line(&self) -> bool {
        matches!(self, Token::Line(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Token::Block(_))
    }
}

/// Tokenizes the lines of a file. It returns a vector of tokens
pub fn tokenize<I>(lines: I) -> Result<Vec<Token>, io::Error>
where
    I: IntoIterator<Item = Result<String, io::Error>>,
{
    let mut tokens = Vec::new();
    let mut in_code_block = false;
    let mut in_front_matter = false;
    let mut code_block = String::new();
    let mut front_matter = String::new();

    for line_result in lines {
        let line = line_result?; // Propagate the error if any line read fails

        // Handle front matter block (enclosed by "---" at the start of the file)
        if in_front_matter {
            if line.trim_start() == "---" {
                in_front_matter = false;
                tokens.push(Token::Block(BlockToken {
                    kind: BlockKind::FrontMatter,
                    content: front_matter.clone(),
                }));
                front_matter.clear();
                continue;
            } else {
                front_matter.push_str(&line);
                front_matter.push('\n');
                continue;
            }
        } else if line.trim_start() == "---" {
            in_front_matter = true;
            continue; // Skip this line, it's part of the front matter delimiter
        }

        // Handle code blocks
        if line.trim_start().starts_with("```") {
            if in_code_block {
                tokens.push(Token::Block(BlockToken {
                    kind: BlockKind::CodeBlock,
                    content: code_block.clone(),
                }));
                code_block.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
            }
        } else if in_code_block {
            code_block.push_str(&line);
            code_block.push('\n');
            continue;
        }

        // Handle headers (lines starting with #)
        if line.starts_with("#") {
            let level = line.chars().take_while(|&c| c == '#').count();
            let content = line[level..].trim().to_string();
            tokens.push(Token::Line(LineToken { level, content }));
        }
        // Handle inline elements (words like normal text, inline code, etc.)
        else {
            let words = tokenize_words(&line);
            tokens.extend(words);
        }
    }

    // Handle any remaining code block at the end
    if in_code_block {
        tokens.push(Token::Block(BlockToken {
            kind: BlockKind::CodeBlock,
            content: code_block,
        }));
    }

    Ok(tokens)
}

/// Tokenizes a line into individual words, identifying inline code, bold, and normal words
fn tokenize_words(line: &str) -> Vec<Token> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut inline_code = false;
    let mut bold = false;
    let mut italic = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '`' => {
                // Inline code logic
                if inline_code {
                    push_word(&mut words, &mut word, WordKind::InlineCode);
                    inline_code = false;
                } else {
                    // Collect content inside backticks as a single word token
                    let mut inline_code_content = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c == '`' {
                            chars.next(); // Skip the closing '`' for inline code
                            break;
                        }
                        inline_code_content.push(chars.next().unwrap());
                    }
                    push_word(&mut words, &mut inline_code_content, WordKind::InlineCode);
                }
            }
            '*' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '*' {
                        // Bold logic (handle `**`)
                        if bold {
                            push_word(&mut words, &mut word, WordKind::Bold);
                            bold = false;
                        } else {
                            // Collect content inside `**` as a single word token
                            let mut bold_content = String::new();
                            chars.next(); // Skip the second '*' in `**`
                            while let Some(&next_c) = chars.peek() {
                                if next_c == '*' {
                                    chars.next(); // Skip the '*' that ends the bold
                                    if let Some(&next_next_c) = chars.peek() {
                                        if next_next_c == '*' {
                                            chars.next(); // Skip the second '*' to complete the bold
                                            break;
                                        }
                                    }
                                }
                                bold_content.push(chars.next().unwrap());
                            }
                            push_word(&mut words, &mut bold_content, WordKind::Bold);
                        }
                    } else if italic {
                        push_word(&mut words, &mut word, WordKind::Italic);
                        italic = false;
                    } else {
                        push_word(&mut words, &mut word, WordKind::Normal);
                        italic = true;
                    }
                }
            }
            ' ' => {
                // Space logic
                push_word(&mut words, &mut word, WordKind::Normal);
                word.clear(); // Prepare for next word
            }
            _ => word.push(c),
        }
    }

    // Handle remaining word
    if !word.is_empty() {
        words.push(Token::Word(WordToken {
            text: word,
            kind: WordKind::Normal,
        }));
    }

    words
}

/// Helper function to push words to the token list
fn push_word(words: &mut Vec<Token>, word: &mut String, kind: WordKind) {
    if !word.is_empty() {
        words.push(Token::Word(WordToken {
            text: word.clone(),
            kind,
        }));
        word.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_heading() {
        let input = "# Heading 1";
        let tokens = tokenize(input.lines().map(|line| Ok(line.to_string()))).unwrap();
        assert_eq!(
            tokens,
            vec![Token::Line(LineToken {
                level: 1,
                content: "Heading 1".to_string(),
            })]
        );
    }

    #[test]
    fn test_tokenize_paragraph() {
        let input = "This is a paragraph.";
        let tokens = tokenize(input.lines().map(|line| Ok(line.to_string()))).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word(WordToken {
                    text: "This".to_string(),
                    kind: WordKind::Normal,
                }),
                Token::Word(WordToken {
                    text: "is".to_string(),
                    kind: WordKind::Normal,
                }),
                Token::Word(WordToken {
                    text: "a".to_string(),
                    kind: WordKind::Normal,
                }),
                Token::Word(WordToken {
                    text: "paragraph.".to_string(),
                    kind: WordKind::Normal,
                }),
            ]
        );
    }

    #[test]
    fn test_tokenize_code_block() {
        let input = "```\nfn main() {}\n```";
        let tokens = tokenize(input.lines().map(|line| Ok(line.to_string()))).unwrap();
        assert_eq!(
            tokens,
            vec![Token::Block(BlockToken {
                kind: BlockKind::CodeBlock,
                content: "fn main() {}\n".to_string(),
            })]
        );
    }

    #[test]
    fn test_tokenize_inline_code() {
        let input = "This `inline code` example.";
        let tokens = tokenize(input.lines().map(|line| Ok(line.to_string()))).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word(WordToken {
                    text: "This".to_string(),
                    kind: WordKind::Normal,
                }),
                Token::Word(WordToken {
                    text: "inline code".to_string(),
                    kind: WordKind::InlineCode,
                }),
                Token::Word(WordToken {
                    text: "example.".to_string(),
                    kind: WordKind::Normal,
                }),
            ]
        );
    }
}
