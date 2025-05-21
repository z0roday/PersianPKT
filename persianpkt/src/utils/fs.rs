use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;

pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn remove_dir_contents(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path)?;
        } else {
            fs::remove_file(entry_path)?;
        }
    }

    Ok(())
}

pub fn copy_dir_contents(src: &Path, dst: &Path) -> Result<()> {
    ensure_dir_exists(dst)?;

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src)?;
        let target_path = dst.join(relative_path);

        if path.is_dir() {
            ensure_dir_exists(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                ensure_dir_exists(parent)?;
            }
            fs::copy(path, target_path)?;
        }
    }

    Ok(())
}

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn get_dir_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            total_size += get_file_size(path)?;
        }
    }

    Ok(total_size)
}

pub fn find_files_by_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(files)
}

pub fn find_files_by_name(dir: &Path, name: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name == name {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(files)
} 