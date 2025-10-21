fn process_scores(scores: &Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();

    for score in scores.iter() {
        if score > &80 {
            results.push(score.clone());
        }
    }

    return results;
}

fn main() {
    let scores = vec![75, 82, 69, 90, 78, 56, 72, 66, 85, 77];
    let high_scores = process_scores(&scores);
    println!("{:?}", high_scores);
}
