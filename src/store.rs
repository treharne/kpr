use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use rev_buf_reader::RevBufReader;

// use crate::helpers::format_line_result_for_output;
use crate::locks::LockGuard;

pub const STORE_FILENAME: &str = "store.txt";

pub fn full_path<S>(filename: S) -> PathBuf 
    where PathBuf: From<S> {
        let filename = PathBuf::from(filename);
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".kpr").join(filename)
}

pub fn open_or_create<S>(filepath: S, append: bool) -> Result<File, std::io::Error> 
    where PathBuf: From<S> {
        let filepath = PathBuf::from(filepath);
        let mut options = OpenOptions::new();
        let options = options
            .read(true)
            .write(true)
            .create(true);


        let options = if append { options.append(true) } else { options.truncate(true) };
        
        options.open(filepath)
}

pub fn open_read<S>(filepath: S) -> Result<File, std::io::Error> 
    where PathBuf: From<S> {
        let filepath = PathBuf::from(filepath);
        OpenOptions::new()
            .read(true)
            .open(filepath)
}


pub fn load_lines_from(file: File, n: Option<usize>) -> Vec<String> {
    let lines_from_last = RevBufReader::new(file)
        .lines()
        .filter_map(|line| line.ok());
        // .filter_map(format_line_result_for_output);

    let lines: Vec<String> = match n {
        Some(n_lines) => lines_from_last.take(n_lines).collect(),
        None => lines_from_last.collect(),
    };

    lines
        .into_iter()
        .rev()
        .collect()
}


pub fn load_lines(n: Option<usize>) -> Vec<String> {
    let filepath = full_path(STORE_FILENAME);
    let file = open_read(filepath).expect("Could not open store file");
    load_lines_from(file, n)
}

pub fn write(text: &str) -> Result<u16, std::io::Error> {
    let filepath = full_path(STORE_FILENAME);
    let file = open_or_create(filepath, true)?;

    let _lock_guard = LockGuard::new(&file)?;

    let reader = BufReader::new(file.try_clone()?);
    let line_count = reader.lines().count() as u16;

    writeln!(&mut file.try_clone()?, "{text}")?;

    // The file will be unlocked when _lock_guard goes out of scope, even if an error occurs.
    Ok(line_count)
}

// Unit tests for the store module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_or_create_creates_file_if_it_does_not_exist() {
        let filepath = full_path("test_open_or_create_creates_file_if_it_does_not_exist.txt");

        let _ = std::fs::remove_file(&filepath);

        assert_eq!(filepath.exists(), false);

        open_or_create(&filepath, false).expect("Could not open store file");

        assert_eq!(filepath.exists(), true);
    }

    #[test]
    fn test_open_or_create_truncates_file_if_it_exists() {
        let filepath = full_path("test_open_or_create_truncates_file_if_it_exists.txt");

        let _ = std::fs::remove_file(&filepath);

        assert_eq!(filepath.exists(), false);

        open_or_create(&filepath, false).expect("Could not open store file");

        assert_eq!(filepath.exists(), true);

        // Open the file again and write some text to it.
        let mut file = open_or_create(&filepath, false).expect("Could not open store file");
        writeln!(&mut file, "Hello, world!").expect("Could not write to file");

        // Open the file again and check that it is empty.
        let file = open_or_create(&filepath, false).expect("Could not open store file");
        let reader = BufReader::new(file);
        let line_count = reader.lines().count();
        assert_eq!(line_count, 0);
    }

    #[test]
    fn test_open_or_create_appends_to_file_if_it_exists() {
        let filepath = full_path("test_open_or_create_appends_to_file_if_it_exists.txt");

        let _ = std::fs::remove_file(&filepath);

        assert_eq!(filepath.exists(), false);

        open_or_create(&filepath, false).expect("Could not open store file");

        assert_eq!(filepath.exists(), true);

        // Open the file again and write some text to it.
        let mut file = open_or_create(&filepath, true).expect("Could not open store file");
        writeln!(&mut file, "Hello, world!").expect("Could not write to file");

        // Open the file again and check that it is empty.
        let file = open_or_create(&filepath, true).expect("Could not open store file");
        let reader = BufReader::new(file);
        let line_count =  reader.lines().count();
        assert_eq!(line_count, 1);
    }
}