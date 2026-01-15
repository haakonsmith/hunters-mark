use crate::config::Config;
use crate::error::Result;
use std::path::PathBuf;

pub fn add(name: Option<String>, path: Option<PathBuf>, tags: Vec<String>) -> Result<()> {
    let mut config = Config::load()?;

    // Use current directory if no path specified
    let path = match path {
        Some(p) => p.canonicalize()?,
        None => std::env::current_dir()?,
    };

    let name = name.unwrap_or_else(|| path.file_name().unwrap().to_string_lossy().into_owned());

    config.add_mark(name.clone(), path.clone(), tags)?;
    config.save()?;

    println!("✓ Added mark {}", path.display());

    Ok(())
}
