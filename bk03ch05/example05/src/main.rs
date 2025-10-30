macro_rules! print_em {
    ( $( $x:expr ),* ) => {
        $(
            print!("{}", $x);
        )*
    };
}

fn main() {
    print_em!(1, 2, 3, 'a', 'b', 'c');
    println!();
    print_em!(10, 20, 30);
    println!();
    print_em!(2, 3, 5, 7, 11, 13, 17, 19);
    println!();
}
