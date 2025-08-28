fn cost_per_slice(total_cost: f64, slices: u32) -> Option<f64> {
    if slices == 0 {
        None
    } else {
        Some(total_cost / slices as f64)
    }
}

fn main() {
    let total_cost = 19.99;
    let slices = 6;
    let result = cost_per_slice(total_cost, slices);
    match result {
        Some(cost) => println!("Each slice costs ${:.2}", cost),
        None => println!("No pizza for you!"),
    }
}
