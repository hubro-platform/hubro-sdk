use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StepsRecord {
    pub startTime: String,
    pub endTime: String,
    pub count: u32,
}