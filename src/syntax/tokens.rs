#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Special
    Illegal,
    // Multiline
    FrontMatter,
    CodeBlock,
    // Single line
    Heading(HeadingToken),
    // Slices
    Word, // Normal words + bolds / italics
    InlineCode,
    Link,
    ListStart,
    ListEnd,
    // Extras
    Newline,
    Whitespace,
    Comment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeadingToken {
    pub level: u8,
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
