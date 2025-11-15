use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

/// Tidy — a small multi-command tool for cleaning and inspecting directories.
#[derive(Parser)]
#[command(version, about = "Keep your folders clean and organized")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory and list all files
    Scan { path: PathBuf },

    /// Archive files older than N days
    Archive {
        path: PathBuf,
        #[arg(long)]
        days: u64,
    },

    /// Display directory statistics
    Stats { path: PathBuf },
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line arguments
    let cli = Cli::parse();

    // Route the command to the appropriate function
    match cli.command {
        Commands::Scan { path } => scan_dir(path)?,
        Commands::Archive { path, days } => archive_old(path, days)?,
        Commands::Stats { path } => stats(path)?,
    }

    Ok(())
}

fn scan_dir(path: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("{}", format!("Scanning {}", path.display()).bold());

    // Iterate through a new WalkDir (recursive directory traversal)
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        // Only handle files
        if entry.file_type().is_file() {
            // Get the file name
            let file_name = entry.file_name().to_string_lossy();

            // Color the files by type
            let colorized = match entry.path().extension().and_then(|e| e.to_str()) {
                Some("rs") => file_name.green(),
                Some("jpg" | "png") => file_name.cyan(),
                Some("zip") => file_name.yellow(),
                _ => file_name.normal(),
            };
            println!("  {}", colorized);
        }
    }
    Ok(())
}

fn archive_old(path: PathBuf, days: u64) -> Result<(), Box<dyn Error>> {
    // Calculate the date before which you want to archive files
    let cutoff = SystemTime::now() - Duration::from_secs(days * 24 * 60 * 60);

    // Set up the archive directory
    let archive_dir = path.join("archive");
    fs::create_dir_all(&archive_dir)?;

    // Create a new WalkDir and store the files in a vector
    let files: Vec<_> = WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    // Initialize the progress bar
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█░"),
    );
    pb.set_prefix("Archiving files");

    // Iterate through the files vector
    for entry in files {
        // Get the file's metadata
        let metadata = entry.metadata()?;

        // Find out when the file was last modified
        if let Ok(modified) = metadata.modified() {
            // If it was last modified prior to the cutoff date, archive it
            if modified < cutoff {
                let dest = archive_dir.join(entry.file_name());
                fs::rename(entry.path(), &dest)?;
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done!");
    Ok(())
}

fn stats(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let mut total_size = 0;

    // Iterate through a new WalkDir
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        // Only handle files
        if entry.file_type().is_file() {
            // Update the file count and the total file size
            count += 1;
            total_size += entry.metadata()?.len();
        }
    }

    // Display the stats
    println!(
        "{}",
        format!(
            "Found {} files, total size {:.2} MB",
            count,
            total_size as f64 / 1_000_000.0
        )
        .bright_yellow()
    );

    Ok(())
}
