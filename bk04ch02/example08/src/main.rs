use csv::Reader;
use std::error::Error;

fn analyze_grades(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a new csv Reader on the file
    let mut reader = Reader::from_path(filename)?;

    // Initialize the analysis variables
    let mut total = 0.0;
    let mut count = 0;

    // Iterate through the records
    for result in reader.records() {
        let record = result?;

        // This CSV has three columns: name, subject, and grade
        let name = &record[0];
        let subject = &record[1];
        let grade: f64 = record[2].parse()?;

        println!("{} got {} in {}", name, grade, subject);
        total += grade;
        count += 1;
    }

    // Display the average grade
    if count > 0 {
        println!("\nClass average: {:.1}", total / count as f64);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    analyze_grades("grades.csv")
}
