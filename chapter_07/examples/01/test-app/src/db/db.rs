pub fn connect(connection_string: &str) -> Result<String, String> {
    if connection_string == "localhost:5432" {
        Ok("Connection successful".to_string())
    } else {
        Err("Unable to connect to database".to_string())
    }
}
