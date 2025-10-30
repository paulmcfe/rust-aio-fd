macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            0 $(+ $x)*
        }
    };
}

fn main() {
    let total5 = sum!(1, 2, 3, 4, 5);
    println!("The 5-item total is: {}", total5);

    let total3 = sum!(1, 2, 3);
    println!("The 3-item total is: {}", total3);

    let total1 = sum!(1);
    println!("The 1-item total is: {}", total1);

    let total0 = sum!();
    println!("The 0-item total is: {}", total0);
}
