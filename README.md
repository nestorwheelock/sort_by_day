If you’re like me, your Downloads folder is a digital wasteland of random files, unorganized PDFs, images with cryptic names, and that one file you swear you downloaded somewhere. Enter File Organizer by Date, a lightweight Rust-based tool I built to bring order to this chaos. It doesn’t overcomplicate things; it scans your files, groups them by their last modified date, and neatly organizes them into dated folders. Simple, efficient, and oh-so-satisfying.
Why File Sorting Matters

In an age where digital clutter accumulates faster than physical clutter, file organization has become more critical than ever. Whether you’re managing a personal computer, overseeing a company’s shared drive, or handling a server full of logs, the ability to quickly find the right file can mean the difference between a smooth workflow and hours of frustration. Proper file sorting isn’t just about aesthetics — it’s about productivity, efficiency, and reducing the mental overhead of sifting through endless file lists.

Unorganized files can lead to duplicated efforts, missed deadlines, and even lost data. For businesses, this can result in financial losses or compliance issues. For individuals, it might mean losing precious memories like photos or failing to submit an important document on time.

Tools like File Organizer by Date take the guesswork out of file management by automating repetitive tasks. Instead of manually sorting files — something that gets increasingly impractical as file volume grows — you can rely on an automated solution to handle it for you.
Why Build Custom Tools?

Not all file organization needs are the same. What works for organizing camera streams might not work for sorting design assets or backup archives. Off-the-shelf tools are great, but they often come with unnecessary features or miss crucial functionality specific to your workflow.

Building custom tools, like File Organizer by Date, allows you to address your exact requirements. Whether you’re a developer building tools for your team or a tech-savvy enthusiast looking to optimize your setup, knowing how to create task-specific utilities is an invaluable skill. Having someone on your team who can write lightweight scripts or programs can save countless hours of manual labor.

For example:

    Creative Teams: Automatically sort assets like images, videos, and project files by modification date.
    Developers: Organize log files from multiple systems into date-based directories.
    System Administrators: Manage backups, server logs, and snapshots with consistent naming conventions.

Custom tools also grow with your needs. Start simple, iterate, and fine-tune the functionality over time.
What Does It Do?

The File Organizer by Date tool does exactly what the name suggests:

    Creates directories by date: Subfolders are named using the format YYYY-MM-DD based on the file's last modified date.
    Moves files: Every file is moved into its corresponding date-based directory.
    Extracts file dates: It reads each file’s metadata to determine its last modified date.

At the end of a run, you’ll see something like:

Moved file: "report.pdf" to "2024-09-03"
Moved file: "notes.txt" to "2024-09-01"

Your mess of files transforms into neatly dated folders. Pure magic.
Why Rust?

Rust is fast, safe, and perfect for tools like this. File handling and date parsing with the chrono crate make building this tool smooth and error-free. Plus, Rust's focus on safety means fewer edge-case surprises.
Getting Started

First, make sure you have Rust installed. If not, grab it from rust-lang.org.
Step 1: Clone the Repository

git clone https://github.com/nestorwheelock/sort_by_day.git
cd sort_by_day

Step 2: Build the Project

cargo build --release

Step 3: Run the Tool

./target/release/file_organizer_by_date /path/to/your/directory

That’s it! Sit back and let the tool work its magic.
How It Works (Simplified)

    The tool scans the provided directory.
    For each file, it checks the last modified date.
    It creates a folder named YYYY-MM-DD if it doesn't already exist.
    It moves the file into the correct folder.

Under the hood, it’s powered by Rust’s std::fs for file handling and chrono for date parsing.
The Source Code Explained

Here’s a quick look at the core logic behind File Organizer by Date:

use std::env;
use std::fs::{self, File};
use std::io::Result;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, NaiveDate};
use std::time::SystemTime;

fn create_day_directory(base_dir: &Path, day: &NaiveDate) -> Result<PathBuf> {
    let day_dir = base_dir.join(day.format("%Y-%m-%d").to_string());
    if !day_dir.exists() {
        fs::create_dir(&day_dir)?;
    }
    Ok(day_dir)
}

fn get_file_day(file_time: SystemTime) -> NaiveDate {
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

This snippet handles creating date-based directories, extracting file modification dates, and moving files into their respective folders.
Need Help With Custom File Sorting or Data Processing?

I specialize in building custom automation tools, data processing scripts, and tailored solutions for file organization. Whether you need help sorting large datasets, automating repetitive tasks, or designing a robust system for managing backups and archives, I can help.

👉 Contact me on Upwork

Let’s collaborate to make your systems secure, efficient, and resilient.
