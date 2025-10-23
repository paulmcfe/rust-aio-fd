//! Utilities for working with financial data.
//!
//! This module defines functions and types for loan payments,
//! investments, future values, and other financial calculations.
///
/// **Calculates the monthly payment for a loan.**
///
/// # Arguments
///
/// - `principal` - The loan amount in dollars
/// - `rate` - *Annual* interest rate as a decimal (0.05 for 5%)
/// - `years` - Length of the loan in years
///
/// # Returns
///
/// The monthly payment amount, rounded to 2 decimal places
/// 
/// # Example
/// 
/// ```
/// let payment = example12::calculate_payment(200000.0, 0.045, 30);
/// assert_eq!(payment, 1013.37);
/// ```
pub fn calculate_payment(principal: f64, rate: f64, years: u32) -> f64 {
    let months = years * 12;
    let monthly_rate = rate / 12.0;

    let payment = principal * (monthly_rate * (1.0 + monthly_rate).powi(months as i32))
        / ((1.0 + monthly_rate).powi(months as i32) - 1.0);

    (payment * 100.0).round() / 100.0
}
