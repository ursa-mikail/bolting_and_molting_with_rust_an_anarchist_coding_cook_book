use validator::validate_email as is_valid_email;

pub fn validate_email(email: &str) -> Result<(), String> {
    if is_valid_email(email) {
        Ok(())
    } else {
        Err("Invalid email address".to_string())
    }
}
