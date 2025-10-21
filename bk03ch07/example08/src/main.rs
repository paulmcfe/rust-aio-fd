fn calculate_discount(price: f64, customer_age: u32) -> f64 {
    let senior_age = 65;
    let senior_discount = 0.15;
    let regular_discount = 0.05;

    let discount_rate = if customer_age >= senior_age {
        senior_discount
    } else {
        regular_discount
    };

    let discount_amount = price * discount_rate;
    price - discount_amount
}

fn main() {
    let final_price = calculate_discount(99.95, 70);
    println!("Final price: ${:.2}", final_price);
}
