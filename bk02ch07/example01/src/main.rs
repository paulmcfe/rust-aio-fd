mod greetings {
    pub fn say_hello() {
        println!("Hello form an inline module!");
    }
}
fn main() {
    greetings::say_hello();
}
