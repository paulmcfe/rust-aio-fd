fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0 // Invalid month
    }
}

fn is_valid_date(day: i32, month: i32, year: i32) -> bool {
    if day < 1 {
        return false;
    }

    if month < 1 || month > 12 {
        return false;
    }

    day <= days_in_month(month, year)
}

fn main() {
    println!("Is 2/29/2028 a valid date? {}", is_valid_date(29, 2, 2028));
    println!("Is 0/1/2027 a valid date? {}", is_valid_date(1, 0, 2027));
    println!("Is 4/31/2026 a valid date? {}", is_valid_date(31, 4, 2026));
    println!("Is 8/23/2025 a valid date? {}", is_valid_date(23, 8, 2025));
}
