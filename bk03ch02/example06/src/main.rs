#![allow(unused_variables)]
#![allow(dead_code)]

struct Report<T, U> {
    title: String,
    data: T,
    id: U,
}

fn main() {
    // Generate a report where the data is numeric
    let financial_report = Report {
        title: String::from("Q2 Sales"),
        data: 1_492_500,
        id: 2026.2,
    };

    // Generate a report where the data is text
    let event_report = Report {
        title: String::from("Conference Summary"),
        data: String::from("The conference was a rousing success..."),
        id: "conference-summary",
    };
}
