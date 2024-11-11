use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingPoint {
    pub bpm: f64,
    pub offset: f64,
    pub inherits: bool,
}