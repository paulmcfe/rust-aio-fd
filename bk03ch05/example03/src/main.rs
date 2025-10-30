macro_rules! toggle {
    (on) => {
        true
    };
    (off) => {
        false
    };
}

fn main() {
    let switch1 = toggle!(on);
    let switch2 = toggle!(off);
    println!("switch1 = {switch1}, switch2 = {switch2}");
}
