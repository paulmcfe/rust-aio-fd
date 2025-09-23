fn get_longer<'a>(text1: &'a str, text2: &'a str) -> &'a str {
    if text1.len() > text2.len() {
        return text1
    }
    text2
}

fn main() {
    let str1 = "It was the best of times";
    let str2 = "It was the worst of times";
    let longer = get_longer(&str1, &str2);
    println!("The longer of the two is '{longer}'.");
}
