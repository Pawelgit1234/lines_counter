use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, BufRead, BufReader};

pub fn count_lines_in_file(file_path: &PathBuf) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let line_count = reader.lines().count();
    Ok(line_count)
}

pub fn count_lines_in_dir(path: &PathBuf, is_recursive: bool) -> io::Result<usize> {
    let mut line_count: usize = 0;

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                line_count += count_lines_in_file(&entry_path)?;
            } else if entry_path.is_dir() && is_recursive {
                line_count += count_lines_in_dir(&entry_path, true)?;
            }
        }
    }

    Ok(line_count)
}