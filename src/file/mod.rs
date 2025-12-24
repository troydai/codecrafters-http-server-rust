/*
 * This module provides components that offer file serving.
 */

use std::{fmt, path::PathBuf, str::FromStr};

pub trait FileRetriever {
    fn retrieve(&self, path: &str) -> Result<Vec<u8>, FileRetrieverError>;
}

#[derive(Debug)]
pub enum FileRetrieverError {
    NotFound,
    Other(String),
}

impl fmt::Display for FileRetrieverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "file not found"),
            Self::Other(msg) => write!(f, "file retriever error: {msg}"),
        }
    }
}

impl std::error::Error for FileRetrieverError {}

struct LocalFileSystem {
    root: PathBuf,
}

struct Dummy {}

impl FileRetriever for LocalFileSystem {
    fn retrieve(&self, path: &str) -> Result<Vec<u8>, FileRetrieverError> {
        let mut full_path = self.root.clone();
        full_path.push(path);

        if !full_path.exists() {
            return Err(FileRetrieverError::NotFound);
        }

        let path = full_path.as_path();
        std::fs::read(path).map_err(|e| {
            FileRetrieverError::Other(format!("failed to read file at {}: {e}", path.display()))
        })
    }
}

impl FileRetriever for Dummy {
    fn retrieve(&self, _path: &str) -> Result<Vec<u8>, FileRetrieverError> {
        Err(FileRetrieverError::Other(String::from("not implemented")))
    }
}

pub fn create(input: Option<String>) -> anyhow::Result<Box<dyn FileRetriever + Send + Sync>> {
    if let Some(path) = input {
        let root = validate_path(&path)?;
        return Ok(Box::new(LocalFileSystem { root }));
    }

    Ok(Box::new(Dummy {}))
}

fn validate_path(s: &str) -> anyhow::Result<PathBuf> {
    if !s.starts_with('/') {
        anyhow::bail!("The directory path is not started from root.")
    }

    if s.contains("..") {
        anyhow::bail!("The directory path contains '..' which is disallowed.")
    }

    let path = PathBuf::from_str(s)?;

    if !path.is_absolute() {
        anyhow::bail!("The path '{s}' is not a valid absolute path.")
    }

    if !path.exists() {
        anyhow::bail!("The directory '{s}' does not exist.")
    }

    if !path.is_dir() {
        anyhow::bail!("The path '{s}' is not a directory.")
    }

    Ok(path)
}
