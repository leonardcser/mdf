use super::tokens::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    // Toc,
    Heading { level: usize, text: String },
    Paragraph { words: Vec<Token> },
}

impl Stmt {
    /// Converts a `Stmt` into a Markdown string.
    pub fn to_markdown(&self) -> String {
        match self {
            Stmt::Heading { level, text } => {
                format!("{} {}", "#".repeat(*level), text)
            }
            Stmt::Paragraph { words } => words
                .iter()
                .map(|token| token.text.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}
