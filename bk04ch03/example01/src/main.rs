use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "notes")]
#[command(about = "A simple note-taking CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    Add {
        /// The note content
        text: String,

        /// Optional tag for categorization
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// List all notes
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Delete a note by its ID
    Delete {
        /// Note ID to delete
        id: usize,
    },
}

fn main() {
    // Parse the command line
    let cli = Cli::parse();

    // Pattern-match the subcommand
    match cli.command {
        // Add a note
        Commands::Add { text, tag } => {
            println!("Adding note: {}", text);
            if let Some(t) = tag {
                println!("  Tag: {}", t);
            }
        }
        // List the notes
        Commands::List { tag } => {
            println!("Listing notes");
            if let Some(t) = tag {
                println!("  Filtered by tag: {}", t);
            }
        }
        // Delete a note
        Commands::Delete { id } => {
            println!("Deleting note with ID: {}", id);
        }
    }
}
