fn main() {
    let score = 87;

    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 50 {
        'D'
    } else {
        0
    };

    println!("Your grade: {grade}");
}
