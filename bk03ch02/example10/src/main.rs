fn clamp<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
    if value < min {return min}
    else if value > max {return max}
    value
}

fn main() {
    let i = 5;
    let f = 1.6;
    let c = "d";
    println!("{}", clamp(i, 5, 10));
    println!("{}", clamp(f, 0.0, 1.0));
    println!("{}", clamp(c, "a", "m"));
}