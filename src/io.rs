use crate::syntax::lexer::Lexer;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};

/// Processes a single file (line by line tokenization)
pub fn process_file(file: &Path) -> io::Result<()> {
    let mut file = File::open(file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to open {}: {}", file.display(), e),
        )
    })?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lexer = Lexer::new(&content);
    let tokens = lexer.collect::<Vec<_>>();
    dbg!(tokens);

    Ok(())
}

/// Recursively traverse a directory and return a list of `.md` files
pub fn visit_dirs<P>(dir: P) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively visit subdirectories
            let files = visit_dirs(&path)?;
            result.extend(files);
        } else if let Some(extension) = path.extension() {
            // Check if the file has a `.md` extension
            if extension == "md" {
                result.push(path);
            }
        }
    }
    Ok(result)
}

/// Process a list of files or folders, only processing `.md` files
pub fn process_paths(paths: &[String]) -> io::Result<()> {
    for path_str in paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            // Process folder recursively, filtering only `.md` files
            match visit_dirs(path) {
                Ok(files) => {
                    for file in files {
                        process_file(&file)?;
                    }
                }
                Err(e) => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("Error reading directory '{}': {}", path_str, e),
                    ));
                }
            }
        } else if path.is_file() {
            // Process individual file if it has a `.md` extension
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    process_file(path)?;
                }
            }
        } else {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("Invalid path: {}", path_str),
            ));
        }
    }
    Ok(())
}
