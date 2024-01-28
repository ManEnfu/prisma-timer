use std::time::SystemTime;

use crate::data::SolveTime;

/// A solve.
#[derive(Debug, Clone)]
pub struct SolveData {
    /// The recorded time of the solve.
    pub time: SolveTime,
    /// The timestamp of when the solve was being recorded.
    pub timestamp: SystemTime,
    /// The scramble used in the solve.
    pub scramble: String,
}

impl SolveData {
    pub fn new(time: SolveTime, scramble: String) -> Self {
        Self {
            time,
            timestamp: SystemTime::now(),
            scramble,
        }
    }
}

impl From<&SolveData> for SolveTime {
    fn from(value: &SolveData) -> Self {
        value.time
    }
}
