
# File Organizer by Date

**File Organizer by Date** is a Rust-based tool designed to organize files into directories based on their last modified date. This tool scans a given directory, identifies the last modification date of each file, and then moves the files into subdirectories named after the date they were last modified.

## Features

- **Directory Creation by Date**: Automatically creates a subdirectory named after the last modified date (in the format `YYYY-MM-DD`).
- **File Moving**: Moves files into their respective date-based directories.
- **File Date Extraction**: Extracts the last modified date of each file and organizes files accordingly.

## Requirements

- **Rust toolchain**: The tool is written in Rust, so you'll need Rust installed to compile and run the program.
- The `chrono` crate is required for date/time handling.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/file-organizer-by-date.git
cd file-organizer-by-date
```

### Step 2: Build the Project

```bash
cargo build --release
```

## Usage

The tool organizes files in a directory by moving them into date-based subdirectories. Run the program by providing the path to the directory you want to organize.

### Command

```bash
./target/release/file_organizer_by_date <directory>
```

### Example

```bash
./target/release/file_organizer_by_date /path/to/directory
```

This will:

1. Scan all files in `/path/to/directory`.
2. Create subdirectories based on each file's last modified date (e.g., `2024-09-03`).
3. Move each file to its corresponding date-based directory.

### Example Output

```bash
Moved file: "example.txt" to "2024-09-03"
Moved file: "document.pdf" to "2024-09-01"
```

The files will be moved to directories named after the date they were last modified, such as `2024-09-03/` or `2024-09-01/`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
