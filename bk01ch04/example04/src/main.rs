fn main() {
    let tax_rate = 5.0;
    let total_price = 52.45;

    if tax_rate >= 0.0 && tax_rate < 1.0 {
        let retail_price = total_price / (1.0 + tax_rate);
        println!("The retail price is ${retail_price:.2}.");
    } else {
        println!("Hmm. Please enter a tax rate between 0 and 1.");
    }
}
