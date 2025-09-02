fn main() {
    let strings_you_pluck = "Banjo,Guitar,Harp,Mandolin,Ukulele";
    for instrument in strings_you_pluck.split(',') {
        println!("{instrument}");
    }
}
