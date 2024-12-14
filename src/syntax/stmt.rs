use super::tokens::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    // Toc,
    Heading {
        level: usize,
        content: String,
    },
    Paragraph {
        words: Vec<Token>,
    },
    FrontMatter {
        content: String,
    },
    CodeBlock {
        language: Option<String>,
        content: String,
    },
}

impl Stmt {
    /// Converts a `Stmt` into a Markdown string.
    pub fn to_markdown(&self) -> String {
        match self {
            Stmt::Heading { level, content } => {
                format!("{} {}", "#".repeat(*level), content)
            }
            Stmt::Paragraph { words } => words
                .iter()
                .map(|token| token.text.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            Stmt::FrontMatter { content } => {
                format!("---\n{}---", content)
            }
            Stmt::CodeBlock { language, content } => {
                let lang_str = language.as_ref().map(|l| l.as_str()).unwrap_or("");
                format!("```{}\n{}```", lang_str, content)
            }
        }
    }
}
