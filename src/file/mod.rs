/*
 * This module provides components that offer file serving.
 */

use anyhow::{Result, bail};
use std::{path::PathBuf, str::FromStr};

pub trait FileRetriever {
    fn retrieve(&self, path: &str) -> Result<Vec<u8>>;
}

struct LocalFileSystem {
    root: PathBuf,
}

struct Dummy {}

impl FileRetriever for LocalFileSystem {
    fn retrieve(&self, path: &str) -> Result<Vec<u8>> {
        let mut full_path = self.root.clone();
        full_path.push(path);

        if !full_path.exists() {
            bail!("path {} does not exist", full_path.display());
        }
        let path = full_path.as_path();
        std::fs::read(path).map_err(|e| {
            println!("failed to read file at {}: {e}", path.display());
            anyhow::anyhow!("internal server error")
        })
    }
}

impl FileRetriever for Dummy {
    fn retrieve(&self, _path: &str) -> Result<Vec<u8>> {
        bail!("not implemented");
    }
}

pub fn create(input: Option<String>) -> Result<Box<dyn FileRetriever + Send + Sync>> {
    if let Some(path) = input {
        let root = validate_path(&path)?;
        return Ok(Box::new(LocalFileSystem { root }));
    }

    Ok(Box::new(Dummy {}))
}

fn validate_path(s: &str) -> Result<PathBuf> {
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
