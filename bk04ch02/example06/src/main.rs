use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn replace_in_file(src: &Path, target: &str, replacement: &str) -> io::Result<()> {
    // 1. Read the original content
    let mut input = String::new();
    File::open(src)?.read_to_string(&mut input)?;

    // 2. Transform the content
    let output = input.replace(target, replacement);

    // 3. Write the transformed content to a temporary file
    // tmp is an owned PathBuf with "txt" replaced by "tmp"
    let tmp = src.with_extension("tmp");
    {
        // This creates a scope for the temporary file

        // Create the temporary file
        let mut f = File::create(&tmp)?;

        // Write everything to the file
        // This operation is buffered, therefore...
        f.write_all(output.as_bytes())?;

        // ...make sure everything in the buffer is written to disk
        f.flush()?;
    } // The scope ends here, so the temporary file is automatically closed

    // 4. Rename the temporary file to the original
    fs::rename(&tmp, src)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Get an &Path to the source file
    let src = Path::new("sexist_idioms.txt");

    // Try the replacement, propagate the error if it fails
    replace_in_file(src, "man", "person")?;

    println!("Replacement successful!");
    Ok(())
}
