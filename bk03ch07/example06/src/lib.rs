/// Returns a simple “loyalty score” based on number of purchases and returns.
/// Score floors at 0.
pub fn loyalty_score(purchases: u32, returns: u32) -> u32 {
    let raw = purchases.saturating_sub(returns);
    // Small bonus if the customer made at least one purchase and no returns
    if purchases > 0 && returns == 0 {
        raw + 1
    } else {
        raw
    }
}
