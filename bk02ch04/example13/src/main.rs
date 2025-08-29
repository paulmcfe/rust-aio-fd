fn main() {
    let mut test_scores = vec![85, 77, 62, 91, 48, 80, 71, 66, 75, 53];

    // Calculate the average text score
    let mut sum_of_scores = 0;
    for score in &test_scores {
        sum_of_scores += score;
    }
    let test_average = sum_of_scores as f64 / test_scores.len() as f64;
    println!("Average test score: {test_average}.");

    // Good behavior bonus: 5 extra points!
    for score in &mut test_scores {
        *score += 5;
    }
    println!("{:?}", test_scores)
}
