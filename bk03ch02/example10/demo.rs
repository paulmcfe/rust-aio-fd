// This won't compile - demonstrates why Copy is needed
fn clamp_without_copy<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {return min}        // value is moved here
    else if value > max {return max}   // ERROR: value already moved!
    value                              // ERROR: value already moved!
}

// This works - with Copy trait
fn clamp_with_copy<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
    if value < min {return min}        // value is copied, original remains
    else if value > max {return max}   // value is copied again
    value                              // value is copied for return
}