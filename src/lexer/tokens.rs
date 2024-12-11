use std::{iter::Peekable, str::CharIndices};

enum TokenKind {
    // Special
    EOF,
    Invalid,
    // Multiline
    FrontMatter,
    CodeBlock,
    // Single line
    Heading,
    // Slices
    Word,
    InlineCode,
    Link,
    ListStart,
    ListEnd,
}

struct Token<'s> {
    kind: TokenKind,
    text: &'s str,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum Token {
//     Paragraph(ParagraphToken),
//     Heading(HeadingToken),
//     Block(BlockToken),
//     List(ListToken),
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct WordToken {
//     pub text: String,
//     pub kind: WordKind,
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum WordKind {
//     Normal,
//     InlineCode,
//     Bold,
//     Italic,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct ParagraphToken {
//     pub words: Vec<WordToken>,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct HeadingToken {
//     pub level: usize,
//     pub content: String,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct BlockToken {
//     pub kind: BlockKind,
//     pub content: String,
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum BlockKind {
//     CodeBlock,
//     FrontMatter,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct ListToken {
//     pub level: usize,
//     pub items: Vec<ListItem>,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct ListItem {
//     pub kind: ListKind,
//     pub content: Vec<Token>,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub enum ListKind {
//     // Ordered,
//     Unordered,
// }
