use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

/// A CLI tool to replace text in a file
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Preview changes without writing to file
    #[arg(short, long)]
    preview: bool,

    /// The string to replace
    from: String,

    /// The string to replace it with
    to: String,

    /// The file to process
    filepath: std::path::PathBuf,
}

fn replace_in_text(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

fn process_file(filepath: &Path, from: &str, to: &str, preview: bool) -> io::Result<usize> {
    // Read the file
    let contents = fs::read_to_string(filepath)?;

    // Count the number of occurrences to be replaced
    let occurrences = contents.matches(from).count();

    if occurrences > 0 {
        let replaced_contents = replace_in_text(&contents, from, to);

        // Is this just a preview?
        if preview {
            eprintln!(
                "This will replace {} occurrences in {:?}",
                occurrences, filepath
            );
        } else {
            // Create a backup copy of the file
            let backup_path = format!("{}.backup", filepath.display());
            fs::copy(filepath, &backup_path)?;

            // Write the replaced contents
            fs::write(filepath, replaced_contents)?;

            eprintln!(
                "Replaced {} occurrences in {:?} (backup: {})",
                occurrences, filepath, backup_path
            );
        }
    } else {
        eprintln!("No occurrences found in {:?}", filepath);
    }

    Ok(occurrences)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let filepath = &args.filepath;

    // Check if file exists and is actually a file
    if !filepath.exists() {
        eprintln!("Error: {:?} does not exist.", filepath);
        std::process::exit(1);
    }

    if !filepath.is_file() {
        eprintln!("Error: {:?} is not a file.", filepath);
        std::process::exit(1);
    }

    // Attempt the replacement
    match process_file(filepath, &args.from, &args.to, args.preview) {
        Ok(count) => {
            if count > 0 && !args.preview {
                println!("\nSummary: {} replacements in {:?}", count, filepath);
            }
        }
        Err(error) => {
            eprintln!("Error processing {:?}: {}", filepath, error);
        }
    }

    Ok(())
}
