// filepath: crates/backend/src/validation.rs
use regex::Regex;

pub struct Validator;

impl Validator {
    /// Validates a username (alphanumeric, dash, underscore, 3-32 chars)
    pub fn validate_username(username: &str) -> Result<(), String> {
        if username.len() < 3 || username.len() > 32 {
            return Err("Username must be 3-32 characters long".to_string());
        }

        if !Regex::new(r"^[a-zA-Z0-9_-]+$")
            .unwrap()
            .is_match(username)
        {
            return Err("Username can only contain alphanumeric characters, dashes, and underscores".to_string());
        }

        Ok(())
    }

    /// Validates an email address
    pub fn validate_email(email: &str) -> Result<(), String> {
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();

        if !email_regex.is_match(email) {
            return Err("Invalid email address".to_string());
        }

        Ok(())
    }

    /// Validates a repository name
    pub fn validate_repo_name(name: &str) -> Result<(), String> {
        if name.is_empty() || name.len() > 100 {
            return Err("Repository name must be 1-100 characters long".to_string());
        }

        if !Regex::new(r"^[a-zA-Z0-9._-]+$")
            .unwrap()
            .is_match(name)
        {
            return Err("Repository name can only contain alphanumeric characters, dots, dashes, and underscores".to_string());
        }

        Ok(())
    }

    /// Validates a password
    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_numeric()) {
            return Err("Password must contain at least one number".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(Validator::validate_username("john_doe-123").is_ok());
    }

    #[test]
    fn test_invalid_username_too_short() {
        assert!(Validator::validate_username("ab").is_err());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Validator::validate_email("notanemail").is_err());
    }

    #[test]
    fn test_valid_email() {
        assert!(Validator::validate_email("test@example.com").is_ok());
    }

    #[test]
    fn test_valid_repo_name() {
        assert!(Validator::validate_repo_name("my-repo.git").is_ok());
    }

    #[test]
    fn test_weak_password() {
        assert!(Validator::validate_password("weak").is_err());
    }

    #[test]
    fn test_valid_password() {
        assert!(Validator::validate_password("StrongPass123").is_ok());
    }
}
