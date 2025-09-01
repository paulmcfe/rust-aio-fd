fn main() {
    let area = {         // The block starts here
        let width = 5;   // width has block scope
        let height = 4;  // height has block scope
        width * height        // This works
    };                        // The block ends here
    
    println!("{width} * {height} = {area}"); // Error!
}
