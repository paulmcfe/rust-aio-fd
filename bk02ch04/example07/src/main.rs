fn normalize_rgb(r: u8, g: u8, b: u8) -> [f32; 3] {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

fn main() {
    let rgb = normalize_rgb(221, 160, 221); // Plum
    println!("Normalized [0-1] RGB values: {:.2?}", rgb);
}
