fn price_per_unit(price: f64, units: f64) -> Result<f64, String> {
    if units == 0.0 {
        Err(String::from("Hey, whoa, you can't divide by zero!"))
    } else {
        Ok(price / units)
    }
}

fn main() {
    let result1 = price_per_unit(18.95, 50.0);
    println!("{:?}", result1);

    let result2 = price_per_unit(32.95, 0.0);
    println!("{:?}", result2);
}
