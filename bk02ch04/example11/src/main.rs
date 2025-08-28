fn main() {
    let test_scores = vec![85, 77, 62, 91, 48, 80, 71, 66, 75, 53];
    // Uncomment the following line to induce a panic
    // println!("Last score: {}", test_scores[10]);

    match test_scores.get(9) {
        Some(score) => println!("Last score: {}", score),
        None => println!("Score not found!"),
    }

    if let Some(score) = test_scores.get(9) {
        println!("Last score: {}", score);
    }
}
