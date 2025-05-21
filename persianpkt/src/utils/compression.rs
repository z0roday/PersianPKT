use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use flate2::read::{GzDecoder};
use flate2::write::{GzEncoder};
use flate2::Compression;
use tar::Archive;
use tar::Builder;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

pub enum CompressionFormat {
    Gzip,
    Xz,
    Plain,
}

impl CompressionFormat {
    pub fn from_extension(path: &Path) -> Self {
        if let Some(extension) = path.extension() {
            match extension.to_str() {
                Some("gz") => Self::Gzip,
                Some("xz") => Self::Xz,
                _ => Self::Plain,
            }
        } else {
            Self::Plain
        }
    }
}

pub fn extract_archive(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let format = CompressionFormat::from_extension(archive_path);
    let file = File::open(archive_path)?;
    
    match format {
        CompressionFormat::Gzip => {
            let decoder = GzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.unpack(target_dir)?;
        }
        CompressionFormat::Xz => {
            let decoder = XzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.unpack(target_dir)?;
        }
        CompressionFormat::Plain => {
            let mut archive = Archive::new(file);
            archive.unpack(target_dir)?;
        }
    }
    
    Ok(())
}

pub fn create_archive(source_dir: &Path, archive_path: &Path) -> Result<()> {
    let format = CompressionFormat::from_extension(archive_path);
    let file = File::create(archive_path)?;
    
    match format {
        CompressionFormat::Gzip => {
            let encoder = GzEncoder::new(file, Compression::default());
            let mut builder = Builder::new(encoder);
            add_directory_to_archive(&mut builder, source_dir, PathBuf::new())?;
            builder.finish()?;
        }
        CompressionFormat::Xz => {
            let encoder = XzEncoder::new(file, 6);
            let mut builder = Builder::new(encoder);
            add_directory_to_archive(&mut builder, source_dir, PathBuf::new())?;
            builder.finish()?;
        }
        CompressionFormat::Plain => {
            let mut builder = Builder::new(file);
            add_directory_to_archive(&mut builder, source_dir, PathBuf::new())?;
            builder.finish()?;
        }
    }
    
    Ok(())
}

fn add_directory_to_archive<W: Write>(builder: &mut Builder<W>, source_dir: &Path, base_path: PathBuf) -> Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = if base_path.as_os_str().is_empty() {
            PathBuf::from(path.file_name().unwrap())
        } else {
            base_path.join(path.file_name().unwrap())
        };
        
        if path.is_dir() {
            builder.append_dir(&relative_path, &path)?;
            add_directory_to_archive(builder, &path, relative_path)?;
        } else {
            let mut file = File::open(&path)?;
            builder.append_file(&relative_path, &mut file)?;
        }
    }
    
    Ok(())
}

pub fn compress_data(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionFormat::Xz => {
            let mut encoder = XzEncoder::new(Vec::new(), 6);
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionFormat::Plain => {
            Ok(data.to_vec())
        }
    }
}

pub fn decompress_data(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        CompressionFormat::Xz => {
            let mut decoder = XzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        CompressionFormat::Plain => {
            Ok(data.to_vec())
        }
    }
} 