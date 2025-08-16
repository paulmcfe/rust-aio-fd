fn calculate_tax(bill: f64, tax_rate: f64) -> f64 {
    bill * (tax_rate / 100.0)
}

fn calculate_tip(bill: f64, percentage: f64) -> f64 {
    bill * (percentage / 100.0)
}

fn calculate_total(bill: f64, tax_rate: f64, percentage: f64) -> f64 {
    let tax = calculate_tax(bill, tax_rate);
    let tip = calculate_tip(bill, percentage);
    bill + tax + tip
}

fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}
fn main() {
    let bill_amount = 50.0;
    let tax_rate = 8.5;
    let tip_percentage = 18.0;
    let total = calculate_total(bill_amount, tax_rate, tip_percentage);
    println!("Bill amount: {}", format_currency(bill_amount));
    println!("Total with tax and tip: {}", format_currency(total));
}
