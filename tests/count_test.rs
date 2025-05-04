use lines_counter::count::{count_lines_in_file, count_lines_in_dir};
use tempfile::TempDir;
use std::fs::{File, create_dir};
use std::io::Write;

#[test]
fn test_count_lines_in_file() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let mut file = File::create(temp_file.path()).unwrap();
    writeln!(file, "Line 1").unwrap();
    writeln!(file, "Line 2").unwrap();
    writeln!(file, "Line 3").unwrap();

    let line_count = count_lines_in_file(&temp_file.path().to_path_buf()).unwrap();
    assert_eq!(line_count, 3);
}

#[test]
fn test_count_lines_in_dir() {
    let temp_dir = TempDir::new().unwrap();

    let file_path_1 = temp_dir.path().join("file1.txt");
    let mut file_1 = File::create(&file_path_1).unwrap();
    writeln!(file_1, "File 1, Line 1").unwrap();
    writeln!(file_1, "File 1, Line 2").unwrap();

    let file_path_2 = temp_dir.path().join("file2.txt");
    let mut file_2 = File::create(&file_path_2).unwrap();
    writeln!(file_2, "File 2, Line 1").unwrap();

    let line_count = count_lines_in_dir(&temp_dir.path().to_path_buf(), false).unwrap();
    assert_eq!(line_count, 3);
}

#[test]
fn test_count_lines_in_dir_with_recursive_flag() {
    let temp_dir = TempDir::new().unwrap();

    let sub_dir = temp_dir.path().join("subdir");
    create_dir(&sub_dir).unwrap();
    let file_path_1 = sub_dir.join("file1.txt");
    let mut file_1 = File::create(&file_path_1).unwrap();
    writeln!(file_1, "Subdir File, Line 1").unwrap();

    let file_path_2 = temp_dir.path().join("file2.txt");
    let mut file_2 = File::create(&file_path_2).unwrap();
    writeln!(file_2, "Main Dir File, Line 1").unwrap();

    let line_count = count_lines_in_dir(&temp_dir.path().to_path_buf(), true).unwrap();
    assert_eq!(line_count, 2);
}
