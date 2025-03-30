mod auth;
mod db;
mod email;
mod logging;
mod utils;
mod libs;  // If you have something in libs

use logging::logging::Logger;
use auth::auth::authenticate_user;
use db::db::connect;
use email::email::validate_email;

fn main() {
    let logger = Logger::new();
    
    // Check user authentication
    match authenticate_user("username", "password") {
        Ok(true) => {
            match connect("localhost:5432") {
                Ok(msg) => println!("{}", msg),
                Err(e) => logger.error("Failed to connect to DB", &e),
            }
        },
        Ok(false) => println!("Authentication failed!"),
        Err(e) => logger.error("Authentication error", &e),
    }

    // Pass the actual username and password to authenticate_user
    match authenticate_user("admin", "secret") {  // This should pass as valid credentials
        Ok(true) => {
            match connect("localhost:5432") {
                Ok(msg) => println!("{}", msg),
                Err(e) => logger.error("Failed to connect to DB", &e),
            }
        },
        Ok(false) => println!("Authentication failed!"),
        Err(e) => logger.error("Authentication error", &e),
    }    

    // Validate email
    match validate_email("test@example.com") {
        Ok(_) => println!("Email is valid!"),
        Err(e) => println!("{}", e),
    }

    match validate_email("invalid_email") {
        Ok(_) => println!("Email is valid!"),
        Err(e) => println!("{}", e),
    }

}

/*
Attempting to authenticate: username = username, password = password
[ERROR] Authentication error: Invalid credentials
Attempting to authenticate: username = admin, password = secret
Connection successful
Email is valid!
Invalid email address
*/
