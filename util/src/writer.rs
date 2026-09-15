//! Writer for writing data to a file in a specific format.

use std::{
    fs::{self, File},
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;

use crate::timer::BenchmarkResult;

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    fn new(extension: impl AsRef<str>) -> Result<Self> {
        // Write to the current working directory with the specified extension.
        let path = PathBuf::from_str(&format!(
            "benchmark.{}",
            extension.as_ref().trim_matches('.')
        ))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        Ok(Self { file })
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        self.file.write_all(data)?;
        Ok(())
    }
}

/// Trait for CSV entries.
pub trait CsvEntry {
    fn columns() -> Vec<String>;
    fn values(&self) -> Vec<String>;
}

pub struct CsvWriter<T: CsvEntry> {
    file_writer: FileWriter,
    _marker: PhantomData<T>,
}

impl<T: CsvEntry> CsvWriter<T> {
    pub fn new() -> Result<Self> {
        let file_writer = FileWriter::new("csv")?;
        let mut instance = Self {
            file_writer,
            _marker: PhantomData,
        };
        instance.write_line(&T::columns().join(","))?;
        Ok(instance)
    }

    fn write_line(&mut self, line: &str) -> Result<()> {
        self.file_writer.write(line.as_bytes())?;
        self.file_writer.write(b"\n")?;
        Ok(())
    }

    pub fn write_entry(&mut self, entry: &T) -> Result<()> {
        self.write_line(&entry.values().join(","))?;
        Ok(())
    }
}

pub trait Serializable {
    fn to_csv(&self) -> Result<()>;
}

impl<T: AsRef<[BenchmarkResult]>> Serializable for T {
    fn to_csv(&self) -> Result<()> {
        let mut writer = CsvWriter::new()?;
        for result in self.as_ref() {
            writer.write_entry(result)?;
        }
        Ok(())
    }
}
