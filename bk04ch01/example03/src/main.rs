use clap::Parser;

/// A simple program to print words
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Make the output uppercase
    #[arg(long)]
    upper: bool,

    /// The number of times to print
    #[arg(long, default_value_t = 1)]
    times: u32,

    /// The word(s) to print
    #[arg(required = true)]
    words: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // Compose and send the message to stdout
    let message = args.words.join(" ");
    let output = if args.upper {
        message.to_uppercase()
    } else {
        message
    };
    for _ in 0..args.times {
        println!("{output}");
    }
}
