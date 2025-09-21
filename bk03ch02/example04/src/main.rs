#![allow(unused_variables)]
#![allow(dead_code)]

struct Report<T> {
    title: String,
    data: T,
}

fn main() {
    // Generate a report where the data is numeric
    let financial_report = Report {
        title: String::from("Q2 Sales"),
        data: 1_492_500,
    };

    // Generate a report where the data is text
    let event_report = Report {
        title: String::from("Conference Summary"),
        data: String::from("The conference was a rousing success..."),
    };
}
