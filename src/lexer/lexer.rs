use crate::lexer::tokens::*;
use std::iter::Peekable;
use std::str::CharIndices;

struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
    current: Option<(usize, usize)>,
}

// /// Tokenizes the lines of a file. It returns a vector of tokens
// pub fn lex<I>(lines: I) -> Result<Vec<Token>, io::Error>
// where
//     I: IntoIterator<Item = Result<String, io::Error>>,
// {
//     let mut tokens = Vec::new();
//     let mut lines = lines.into_iter();
//
//     while let Some(line_result) = lines.next() {
//         let line = line_result?;
//         let trimmed_line = line.trim_start();
//
//         // Check for front matter
//         if trimmed_line == "---" {
//             tokens.push(consume_block(&mut lines, BlockKind::FrontMatter)?);
//             continue;
//         }
//
//         // Check for code block
//         if trimmed_line.starts_with("```") {
//             tokens.push(consume_block(&mut lines, BlockKind::CodeBlock)?);
//             continue;
//         }
//
//         // Handle headings
//         if line.starts_with("#") {
//             let level = line.chars().take_while(|&c| c == '#').count();
//             let content = line[level..].trim().to_string();
//             tokens.push(Token::Heading(HeadingToken { level, content }));
//             continue;
//         }
//
//         // Handle lists
//         if trimmed_line.starts_with("- ") {
//             tokens.push(consume_list(&mut lines, line)?);
//             continue;
//         }
//
//         // Handle paragraphs
//         if !line.trim().is_empty() {
//             tokens.push(consume_paragraph(&mut lines, line)?);
//             continue;
//         }
//     }
//
//     Ok(tokens)
// }
//
// fn consume_block<I>(lines: &mut I, kind: BlockKind) -> Result<Token, io::Error>
// where
//     I: Iterator<Item = Result<String, io::Error>>,
// {
//     let mut content = String::new();
//
//     while let Some(line_result) = lines.next() {
//         let line = line_result?;
//
//         // Check for block termination
//         if line.trim_start() == "---" || line.trim_start().starts_with("```") {
//             break;
//         }
//
//         content.push_str(&line);
//         content.push('\n');
//     }
//
//     Ok(Token::Block(BlockToken { kind, content }))
// }
//
// fn consume_paragraph<I>(lines: &mut I, first_line: String) -> Result<Token, io::Error>
// where
//     I: Iterator<Item = Result<String, io::Error>>,
// {
//     let mut paragraph_words = lex_words(&first_line);
//
//     while let Some(line_result) = lines.next() {
//         let line = line_result?;
//
//         // Stop if line is empty
//         if line.trim().is_empty() {
//             break;
//         }
//
//         // Extend paragraph words
//         paragraph_words.extend(lex_words(&line));
//     }
//
//     Ok(Token::Paragraph(ParagraphToken {
//         words: paragraph_words,
//     }))
// }
//
// fn consume_list<I>(lines: &mut I, first_line: String) -> Result<Token, io::Error>
// where
//     I: Iterator<Item = Result<String, io::Error>>,
// {
//     let mut list_stack: Vec<ListToken> = Vec::new();
//
//     // Create the first list at the base level
//     let base_indent = first_line.chars().take_while(|c| c.is_whitespace()).count();
//     let first_item_content = first_line.trim_start_matches(|c: char| c == '-' || c.is_whitespace());
//
//     let mut current_list = ListToken {
//         level: base_indent,
//         items: vec![ListItem {
//             kind: ListKind::Unordered,
//             content: vec![Token::Paragraph(ParagraphToken {
//                 words: lex_words(first_item_content),
//             })],
//         }],
//     };
//
//     // Prepare to track the last processed indent
//     let mut last_indent = base_indent;
//
//     // Continue processing subsequent list items
//     while let Some(line_result) = lines.next() {
//         let line = line_result?;
//         let trimmed_line = line.trim_start();
//
//         // Check if line is a list item
//         if trimmed_line.starts_with("- ") || trimmed_line.starts_with('-') {
//             let item_indent = line.chars().take_while(|c| c.is_whitespace()).count();
//             let item_content =
//                 trimmed_line.trim_start_matches(|c: char| c == '-' || c.is_whitespace());
//
//             let new_item = ListItem {
//                 kind: ListKind::Unordered,
//                 content: vec![Token::Paragraph(ParagraphToken {
//                     words: lex_words(item_content),
//                 })],
//             };
//
//             // Determine list nesting
//             if item_indent > last_indent {
//                 // Start a new nested list
//                 let nested_list = ListToken {
//                     level: item_indent,
//                     items: vec![new_item],
//                 };
//
//                 // Add the nested list to the last item of the current list
//                 if let Some(last_item) = current_list.items.last_mut() {
//                     last_item.content.push(Token::List(nested_list.clone()));
//                 }
//
//                 // Keep track of the nested list
//                 list_stack.push(current_list);
//                 current_list = nested_list;
//             } else if item_indent < last_indent {
//                 // Move back up the list hierarchy
//                 while let Some(mut parent_list) = list_stack.pop() {
//                     if item_indent >= parent_list.level {
//                         // Add current list to parent's last item
//                         if let Some(last_item) = parent_list.items.last_mut() {
//                             last_item.content.push(Token::List(current_list));
//                         }
//                         current_list = parent_list;
//
//                         // Add new item to the current list at this level
//                         current_list.items.push(new_item);
//                         break;
//                     }
//                 }
//             } else {
//                 // Same level, just add a new item
//                 current_list.items.push(new_item);
//             }
//
//             last_indent = item_indent;
//         } else if line.trim().is_empty() {
//             // End of list
//             break;
//         } else {
//             // Continuation of previous list item
//             if let Some(last_item) = current_list.items.last_mut() {
//                 // Add additional content to the last list item
//                 last_item.content.push(Token::Paragraph(ParagraphToken {
//                     words: lex_words(line.trim()),
//                 }));
//             }
//         }
//     }
//
//     // Handle any remaining nested lists
//     while let Some(mut parent_list) = list_stack.pop() {
//         if let Some(last_item) = parent_list.items.last_mut() {
//             last_item.content.push(Token::List(current_list));
//         }
//         current_list = parent_list;
//     }
//
//     Ok(Token::List(current_list))
// }
//
// /// Tokenizes a line into individual words, identifying inline code, bold, and normal words
// fn lex_words(line: &str) -> Vec<WordToken> {
//     let mut words = Vec::new();
//     let mut word = String::new();
//     let mut inline_code = false;
//     let mut bold = false;
//     let mut italic = false;
//     let mut chars = line.chars().peekable();
//
//     while let Some(c) = chars.next() {
//         match c {
//             '`' => {
//                 // Inline code logic
//                 if inline_code {
//                     push_word(&mut words, &mut word, WordKind::InlineCode);
//                     inline_code = false;
//                 } else {
//                     // Collect content inside backticks as a single word token
//                     let mut inline_code_content = String::new();
//                     while let Some(&next_c) = chars.peek() {
//                         if next_c == '`' {
//                             chars.next(); // Skip the closing '`' for inline code
//                             break;
//                         }
//                         inline_code_content.push(chars.next().unwrap());
//                     }
//                     push_word(&mut words, &mut inline_code_content, WordKind::InlineCode);
//                 }
//             }
//             '*' => {
//                 if let Some(&next_char) = chars.peek() {
//                     if next_char == '*' {
//                         // Bold logic (handle `**`)
//                         if bold {
//                             push_word(&mut words, &mut word, WordKind::Bold);
//                             bold = false;
//                         } else {
//                             // Collect content inside `**` as a single word token
//                             let mut bold_content = String::new();
//                             chars.next(); // Skip the second '*' in `**`
//                             while let Some(&next_c) = chars.peek() {
//                                 if next_c == '*' {
//                                     chars.next(); // Skip the '*' that ends the bold
//                                     if let Some(&next_next_c) = chars.peek() {
//                                         if next_next_c == '*' {
//                                             chars.next(); // Skip the second '*' to complete the bold
//                                             break;
//                                         }
//                                     }
//                                 }
//                                 bold_content.push(chars.next().unwrap());
//                             }
//                             push_word(&mut words, &mut bold_content, WordKind::Bold);
//                         }
//                     } else if italic {
//                         push_word(&mut words, &mut word, WordKind::Italic);
//                         italic = false;
//                     } else {
//                         push_word(&mut words, &mut word, WordKind::Normal);
//                         italic = true;
//                     }
//                 }
//             }
//             ' ' => {
//                 // Space logic
//                 push_word(&mut words, &mut word, WordKind::Normal);
//                 word.clear(); // Prepare for next word
//             }
//             _ => word.push(c),
//         }
//     }
//
//     // Handle remaining word
//     if !word.is_empty() {
//         words.push(WordToken {
//             text: word,
//             kind: WordKind::Normal,
//         });
//     }
//
//     words
// }
//
// /// Helper function to push words to the token list
// fn push_word(words: &mut Vec<WordToken>, word: &mut String, kind: WordKind) {
//     if !word.is_empty() {
//         words.push(WordToken {
//             text: word.clone(),
//             kind,
//         });
//         word.clear();
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_tokenize_heading_1() {
//         let input = "# Heading 1";
//         let tokens = lex(input.lines().map(|line| Ok(line.to_string()))).unwrap();
//         assert_eq!(
//             tokens,
//             vec![Token::Heading(HeadingToken {
//                 level: 1,
//                 content: "Heading 1".to_string(),
//             })]
//         );
//     }
//
//     #[test]
//     fn test_tokenize_heading_4() {
//         let input = "#### Heading 4";
//         let tokens = lex(input.lines().map(|line| Ok(line.to_string()))).unwrap();
//         assert_eq!(
//             tokens,
//             vec![Token::Heading(HeadingToken {
//                 level: 4,
//                 content: "Heading 4".to_string(),
//             })]
//         );
//     }
//     #[test]
//     fn test_tokenize_code_block() {
//         let input = "```\nfn main() {}\n```";
//         let tokens = lex(input.lines().map(|line| Ok(line.to_string()))).unwrap();
//         assert_eq!(
//             tokens,
//             vec![Token::Block(BlockToken {
//                 kind: BlockKind::CodeBlock,
//                 content: "fn main() {}\n".to_string(),
//             })]
//         );
//     }
// }
