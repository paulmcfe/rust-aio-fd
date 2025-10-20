fn parse_tablespoons(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    
    trimmed
        .parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number", trimmed))
}

#[test]
fn test_valid_tablespoons() {
    assert_eq!(parse_tablespoons("2.5"), Ok(2.5));
    assert_eq!(parse_tablespoons(" 3 "), Ok(3.0));
}

#[test]
fn test_invalid_tablespoons() {
    assert!(parse_tablespoons("two").is_err());
    assert!(parse_tablespoons("").is_err());
}

#[test]
fn test_error_messages() {
    match parse_tablespoons("xyz") {
        Err(msg) => assert!(msg.contains("not a valid number")),
        Ok(_) => panic!("Expected an error but got Ok"),
    }
}

fn main() {
    
}