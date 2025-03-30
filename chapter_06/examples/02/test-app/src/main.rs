mod utils;

use rand::Rng;
use utils::{errors::*, logs::process_log, mission::handle_docking_error, orbit::*};

fn main() {
    let mut rng = rand::thread_rng();

    // === 🚀 Launch Control: Preflight Check ===
    println!("🚀 Launch Control: Preflight Check");
    match preflight_check() {
        Ok(_) => println!("All systems go! Ignition sequence start.\n"),
        Err(e) => {
            println!("Mission aborted: {}", e);
            return;
        }
    }

    // === 🛰️ Orbit Adjustment: Error Retry ===
    println!("🛰️ Orbit Adjustment: Aligning Trajectory");
    let max_retries = 3;
    for attempt in 1..=max_retries {
        match adjust_orbit() {
            Ok(_) => {
                println!("Orbit adjustment successful!");
                break;
            }
            Err(e) => {
                println!("Attempt {} failed: {}", attempt, e);
                if attempt == max_retries {
                    println!("Mission failed: Unable to adjust orbit.");
                    return;
                }
            }
        }
    }

    // === 🤝 Docking Control: Managing Multiple Errors ===
    println!("\n🤝 Docking Control: Preparing for ISS Docking");
    let docking_outcome = rng.gen_range(0..3);
    match docking_outcome {
        0 => handle_docking_error(ERR_FUEL_LEAK),
        1 => handle_docking_error(ERR_NAVIGATION),
        2 => println!("Docking successful! Crew transfer initiated."),
        _ => handle_docking_error(ERR_DOCKING_FAILED),
    }

    // === 📜 Mission Logs: Panic and Recovery ===
    println!("\n📜 Mission Logs: Embrace the Chaos");
    let logs = [
        "Telemetry data received",
        "Solar panel deployed",
        "Hull breach detected",
        "Power system failure",
    ];
    println!("Mission log complete.");
    for log in logs.iter() {
        process_log(log);
    }

    println!("\n🎉 Mission accomplished! Returning to Earth.");
}


/*
🚀 Launch Control: Preflight Check
All systems go! Ignition sequence start.

🛰️ Orbit Adjustment: Aligning Trajectory
Orbit adjustment successful!

🤝 Docking Control: Preparing for ISS Docking
Docking successful! Crew transfer initiated.

📜 Mission Logs: Embrace the Chaos
Mission log complete.
Processing log: Telemetry data received
Processing log: Solar panel deployed
Processing log: Hull breach detected

thread 'main' panicked at src/utils/logs.rs:10:13:
🚨 Critical failure: Hull integrity compromised!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
⚠️ Recovered from catastrophic error: Any { .. }
Processing log: Power system failure

🎉 Mission accomplished! Returning to Earth.
*/