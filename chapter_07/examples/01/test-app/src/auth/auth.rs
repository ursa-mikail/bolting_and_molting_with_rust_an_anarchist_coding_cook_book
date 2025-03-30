pub fn authenticate_user(username: &str, password: &str) -> Result<bool, String> {
    println!("Attempting to authenticate: username = {}, password = {}", username, password);  // Debugging line
    if username == "admin" && password == "secret" {
        Ok(true)
    } else {
        Err("Invalid credentials".to_string())
    }
}
