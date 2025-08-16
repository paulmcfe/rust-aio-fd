fn calculate_tip(bill: f64, percentage: f64) -> f64 {
    bill * (percentage / 100.0)
}

fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

fn main() {
    let bill_amount = 50.0;
    let tip = calculate_tip(bill_amount, 18.0);
    let total = bill_amount + tip;
    
    println!("Bill: {}", format_currency(bill_amount));
    println!("Tip: {}", format_currency(tip));
    println!("Total: {}", format_currency(total));
}