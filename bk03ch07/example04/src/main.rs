fn divide_cookies(cookies: u32, people: u32) -> u32 {
    if people == 0 {
        panic!("Can't divide cookies among zero people!");
    }
    cookies / people
}

#[test]
#[should_panic]
fn test_panic_with_zero_people() {
    divide_cookies(10, 0);  // This should panic
}

#[test]
#[should_panic(expected = "zero people")]
fn test_panic_message() {
    divide_cookies(10, 0);  // Panic message must contain "zero people"
}

fn main() {
    
}