use std::fmt;

#[derive(Debug)]
pub struct MissionError(pub &'static str);

impl fmt::Display for MissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MissionError {}

pub const ERR_FUEL_LEAK: MissionError = MissionError("Critical fuel leak detected");
pub const ERR_NAVIGATION: MissionError = MissionError("Navigation system failure");
pub const ERR_DOCKING_FAILED: MissionError = MissionError("Docking procedure failed");