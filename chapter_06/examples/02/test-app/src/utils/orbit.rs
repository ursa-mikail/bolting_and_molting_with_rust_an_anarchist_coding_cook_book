use rand::Rng;
use crate::utils::errors::MissionError;

// === Preflight Check ===
pub fn preflight_check() -> Result<(), MissionError> {
    let mut rng = rand::thread_rng();
    let chance: f32 = rng.r#gen(); // Using raw identifier syntax
    if chance < 0.2 {
        return Err(MissionError("Preflight sensor malfunction".into())); // Used .into() for string conversion
    }
    Ok(())
}

// === Orbit Adjustment ===
pub fn adjust_orbit() -> Result<(), MissionError> {
    let mut rng = rand::thread_rng();
    let chance: f32 = rng.r#gen(); // Using raw identifier syntax
    if chance < 0.5 {
        return Err(MissionError("Thruster misalignment detected".into()));
    }
    Ok(())
}
