fn calculate_tip(pre_tip: f32, tip_percent: f32) -> f32 {
    pre_tip * tip_percent
}


fn main() {
    let pre_tip_total = 100.00;
    let tip_percentage = 0.18;
    let tip_cost = calculate_tip(pre_tip_total, tip_percentage);
    let total_bill = pre_tip_total + tip_cost;
    println!("Your total bill is ${total_bill:.2}");
}
