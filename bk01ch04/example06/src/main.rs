fn main() {
    let total_price = 11.00;
    let tax_rate = 0.1;
    let retail_price = total_price / (1.0 + tax_rate);
    println!("The pre-tax price is ${retail_price:.2}.");
}
