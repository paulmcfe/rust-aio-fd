fn estimate_reading_time(word_count: u32) -> u32 {
    let words_per_minute = 200;

    // Round up to the nearest minute
    word_count.div_ceil(words_per_minute)
}

#[test]
fn test_exact_minutes() {
    // 400 words at 200 wpm should be exactly 2 minutes
    assert_eq!(estimate_reading_time(400), 2);
}

#[test]
fn test_rounds_up() {
    // 450 words should round up to 3 minutes
    assert_eq!(estimate_reading_time(450), 3);
}

#[test]
fn test_very_short_text() {
    // Even 1 word should show as 1 minute
    assert_eq!(estimate_reading_time(1), 1);
}

#[test]
fn test_zero_words() {
    // Zero words should take no time to read!
    assert_eq!(estimate_reading_time(0), 0);
}

fn main() {
    println!("{}", estimate_reading_time(1000));
}

