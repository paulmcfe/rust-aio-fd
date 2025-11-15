use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "db")]
#[command(about = "A simple database CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// User management commands
    User {
        #[command(subcommand)]
        action: UserCommands,
    },

    /// Database backup commands
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// Create a new user
    Create {
        username: String,
        #[arg(short, long)]
        email: String,
    },

    /// Delete an existing user
    Delete { username: String },
}

#[derive(Subcommand)]
enum BackupCommands {
    /// Create a database backup
    Create {
        #[arg(short, long, default_value = "./backup")]
        output: String,
    },

    /// Restore from a backup
    Restore { backup_file: String },
}

fn main() {
    // Parse the command line
    let cli = Cli::parse();

    // Pattern-match the subcommands
    match cli.command {
        // Pattern-match the User second-level subcommands
        Commands::User { action } => match action {
            // Create a new user
            UserCommands::Create { username, email } => {
                println!("Creating user: {} ({})", username, email);
            }
            // Delete a user
            UserCommands::Delete { username } => {
                println!("Deleting user: {}", username);
            }
        },
        // Pattern-match the Backup second-level subcommands
        Commands::Backup { action } => match action {
            // Create a backup
            BackupCommands::Create { output } => {
                println!("Creating backup at: {}", output);
            }
            // Restore from a backup
            BackupCommands::Restore { backup_file } => {
                println!("Restoring from: {}", backup_file);
            }
        },
    }
}
