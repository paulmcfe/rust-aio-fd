pub struct PasswordValidator {
    min_length: usize,
    require_uppercase: bool,
    require_number: bool,
}

impl PasswordValidator {
    pub fn new(min_length: usize) -> Self {
        PasswordValidator {
            min_length,
            require_uppercase: true,
            require_number: true,
        }
    }
    
    pub fn validate(&self, password: &str) -> Result<(), String> {
        if password.len() < self.min_length {
            return Err(format!("Password must be at least {} characters", 
                             self.min_length));
        }
        
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain an uppercase letter".to_string());
        }
        
        if self.require_number && !password.chars().any(|c| c.is_numeric()) {
            return Err("Password must contain a number".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_password() {
        let validator = PasswordValidator::new(8);
        assert!(validator.validate("Secret123").is_ok());
    }
    
    #[test]
    fn test_too_short() {
        let validator = PasswordValidator::new(8);
        let result = validator.validate("Hi1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least 8"));
    }
    
    #[test]
    fn test_missing_uppercase() {
        let validator = PasswordValidator::new(8);
        let result = validator.validate("secret123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("uppercase"));
    }
    
    #[test]
    fn test_missing_number() {
        let validator = PasswordValidator::new(8);
        let result = validator.validate("SecretPassword");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("number"));
    }
}

fn main() {}