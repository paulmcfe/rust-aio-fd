macro_rules! pairs {
    ( $( $k:literal : $v:expr ),* $(,)? ) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push(($k, $v));
            )*
            tmp
        }
    };
}

fn main() {
    let settings = pairs!(
        "port": 3000,
        "timeout": 5000,
        "retries": 3,
    );

    println!("{:?}", settings);
}
