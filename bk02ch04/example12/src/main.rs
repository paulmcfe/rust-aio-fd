fn main() {
    let mut css_colors = vec!["bisque", "lemonchiffon"];
    css_colors.push("papayawhip");
    println!("{:?}", css_colors);

    css_colors.insert(1, "honeydew");
    println!("{:?}", css_colors);

    if let Some(color) = css_colors.pop() {
        println!("Removed {color}!");
        println!("{:?}", css_colors);
    }

    css_colors.remove(1);
    println!("{:?}", css_colors);
}
