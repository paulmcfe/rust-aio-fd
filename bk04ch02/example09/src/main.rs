use csv::Writer;
use std::error::Error;

fn save_inventory(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a new csv Writer on the file
    let mut writer = Writer::from_path(filename)?;

    // Write the field headings
    writer.write_record(&["Item", "Quantity", "Price"])?;

    // Write the data
    writer.write_record(&["Rubber Duck", "42", "5.99"])?;
    writer.write_record(&["Keyboard", "3", "89.99"])?;
    writer.write_record(&["Coffee Mug", "17", "12.50"])?;

    // Make sure the buffer is cleared and everything is written to the file
    writer.flush()?;

    println!("Inventory saved!");
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    save_inventory("src/inventory.csv")
}
