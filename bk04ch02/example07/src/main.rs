use chrono;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn add_journal_entry(entry: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true) // Create if it doesn't exist
        .append(true) // Add to the end, don't overwrite
        .open("src/journal.txt")?; // Open file with options

    writeln!(
        file,
        "[{}]\n{}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
        entry
    )?;
    Ok(())
}
fn main() -> io::Result<()> {
    add_journal_entry("This is a test entry.")?;
    println!("Journal entry added.");
    Ok(())
}
