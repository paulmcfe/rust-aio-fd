fn set_temperature(temp: f64) -> Result<f64, &'static str> {
    if temp < 0.0 || temp > 100.0 {
        return Err("The temperature must be between 0 and 100!");
    }
    Ok(temp)
}

fn main() {
    println!("Setting the temperature...");

    // Example 1: unwrap_or_else() with Ok value
    let temp1 = 66.0;
    let result1 = set_temperature(temp1);
    let value1 = result1.unwrap_or_else(|err| {
        println!("{err}");
        72.0
    });
    println!(
        "You asked for {temp1} degrees. \
        Temperature set to {value1} degrees."
    );

    // Example 2: unwrap_or() with Err value
    println!();
    println!("Setting the temperature...");
    let temp2 = 105.0;
    let result2 = set_temperature(temp2);
    let value2 = result2.unwrap_or_else(|err| {
        println!("{err}");
        72.0
    });
    println!(
        "You asked for {temp2} degrees. \
        Temperature set to {value2} degrees."
    );
}
