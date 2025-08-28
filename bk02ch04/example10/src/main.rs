fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn main() {
    let a = [1, 2, 3];
    let b = [1, 2, 3, 4];
    println!("{}", sum(&a)); // Output: 6
    println!("{}", sum(&b)); // Output: 10
}
