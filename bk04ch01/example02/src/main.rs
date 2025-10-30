use std::env;
use std::process;

fn print_usage() {
    eprintln!("Usage: example02 [--upper] [--times N] <words...>");
}

fn main() {
    // Get the arguments
    let mut args = env::args();

    // Ignore the program name (first argument)
    let _program = args.next();

    // Program configuration
    let mut upper = false;
    let mut times: u32 = 1;
    let mut words: Vec<String> = Vec::new();

    // Iterate through the rest of the arguments
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--upper" => upper = true,
            "--times" => {
                // Check for no number specified with --times
                let n = args.next().unwrap_or_else(|| {
                    eprintln!("example02: --times requires a number.");
                    process::exit(2);
                });
                // Parse the number of times
                times = n.parse().unwrap_or_else(|_| {
                    eprintln!("example02: invalid number for --times: {n}");
                    process::exit(2);
                });
            }
            // Check for an unknown option
            s if s.starts_with('-') => {
                eprintln!("example02: unknown option: {s}");
                print_usage();
                process::exit(2);
            }
            // Push the word(s) to the vector
            word => words.push(word.to_string()),
        }
    }

    // Check for no words specified
    if words.is_empty() {
        eprintln!("example02: No word(s) specified");
        print_usage();
        process::exit(2);
    }

    // Compose and send the message to stdout
    let message = words.join(" ");
    let output = if upper {
        message.to_uppercase()
    } else {
        message
    };
    for _ in 0..times {
        println!("{output}");
    }
}
