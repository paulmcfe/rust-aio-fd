use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_words_in_file(filename: &str) -> std::io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut total_words = 0;
    
    for line in reader.lines() {
        let line = line?;  // Each line could fail to read
        total_words += line.split_whitespace().count();
    }
    
    Ok(total_words)
}

fn main() {
    match count_words_in_file("book.md") {
        Ok(count) => println!("Your book has {} words. Keep writing!", count),
        Err(e) => println!("Couldn't count words: {}", e),
    }
}