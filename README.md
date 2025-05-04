# Lines Counter

📄 A simple CLI tool to count lines in a file or directory.

## Features

- ✅ Count lines in individual files
- ✅ Count lines in directories
- ✅ Recursive traversal using `-r` flag
- ✅ Fully tested with isolated test files

## Usage

```bash
# Count lines in a single file
cargo run -- path/to/file.rs

# Count lines in a directory (non-recursive)
cargo run -- path/to/directory

# Count lines recursively in a directory
cargo run -- path/to/directory -r
```

## Installation
```bash
git clone https://github.com/Pawelgit1234/lines_counter.git
cd lines_counter
cargo build --release
```