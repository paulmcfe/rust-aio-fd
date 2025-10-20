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

fn main() {

}
