fn main() {
    let month = "January";
    let year = 2026;
    let days_in_month= match month {
        "January" => 31,
        "February" => {
            if year % 4 == 0 {
                29
            }
            else {
                28
            }
        },
        "March" => 31,
        "April" => 30,
        "May" => 31,
        "June" => 30,
        "July" => 31,
        "August" => 31,
        "September" => 30,
        "October" => 31,
        "November" => 30,
        "December" => 31,
        _ => 0
    };

    if days_in_month > 0 {
        println!("{month} has {days_in_month} days.")
    }
    
}
