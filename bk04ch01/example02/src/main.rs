use std::env;
use std::process;

fn print_usage() {
    eprint!("Usage: example02 [--upper] [--times N] <words...>");
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

    //Iterate through the rest of the arguments
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--upper" => upper = true,
            "--times" => {
                let n = args.next().unwrap_or_else(|| {
                    eprint!("example02: --times requires a number.");
                    process::exit(2);
                });
                times = n.parse().unwrap_or_else(|_| {
                    eprint!("example02: invalid number for --times: {n}");
                    process::exit(2);
                });
            }
            s if s.starts_with('-') => {
                eprint!("example02: unknown option: {s}");
                print_usage();
                process::exit(2);
            }
            word => words.push(word.to_string()),
        }

        // Check for no words specified
        if words.is_empty() {
            eprint!("example02: No word(s) specified");
            print_usage();
            process::exit(2);
        }

        let message = words.join(" ");
        let output = if upper {
            message.to_uppercase()
        }
        else {
            message
        };

        for _ in 0..times {
            println!("{output}");
        }
    }
}
