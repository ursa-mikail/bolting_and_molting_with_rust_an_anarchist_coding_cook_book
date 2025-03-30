use crate::utils::errors::MissionError;

// === Handle Docking Errors ===
pub fn handle_docking_error(err: MissionError) {
    match err.0 {
        "Critical fuel leak detected" => println!("❌ Critical error: Fuel leak detected! Abort docking."),
        "Navigation system failure" => println!("❌ Error: Navigation system failure! Switching to manual controls."),
        "Docking procedure failed" => println!("❌ Error: Docking procedure failed. Attempting emergency protocol."),
        _ => println!("❌ Unknown error occurred during docking."),
    }
}

