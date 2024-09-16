use std::env;
use std::fs::{self, File};
use std::io::Result;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, NaiveDate};
use std::time::SystemTime;

fn create_day_directory(base_dir: &Path, day: &NaiveDate) -> Result<PathBuf> {
    // Create a directory inside the base directory named after the date (e.g., "2024-09-03")
    let day_dir = base_dir.join(day.format("%Y-%m-%d").to_string());
    if !day_dir.exists() {
        fs::create_dir(&day_dir)?;
    }
    Ok(day_dir)
}

fn get_file_day(file_time: SystemTime) -> NaiveDate {
    // Convert the file's modification time to a NaiveDate (YYYY-MM-DD)
    let datetime: DateTime<Local> = DateTime::from(file_time);
    datetime.date().naive_local()
}

fn move_file_to_day_directory(file_path: &Path, day_dir: &Path) -> Result<()> {
    let file_name = file_path.file_name().unwrap();
    let dest_path = day_dir.join(file_name);
    fs::rename(file_path, dest_path)?;
    println!("Moved file: {:?} to {:?}", file_name, day_dir);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return Ok(());
    }

    let base_directory = PathBuf::from(&args[1]);

    // Check if the provided directory exists
    if !base_directory.exists() || !base_directory.is_dir() {
        eprintln!("The provided path is not a valid directory.");
        return Ok(());
    }

    for entry in fs::read_dir(&base_directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified_time = metadata.modified()?;

            // Get the day the file was last modified
            let file_day = get_file_day(modified_time);

            // Create a directory for the day inside the base directory
            let day_dir = create_day_directory(&base_directory, &file_day)?;

            // Move the file to the day directory
            move_file_to_day_directory(&path, &day_dir)?;
        }
    }

    Ok(())
}

