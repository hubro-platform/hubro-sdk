use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StepsRecord {
    pub startTime: String,
    pub endTime: String,
    pub count: u32,
}

impl StepsRecord {
    pub const IDENTIFIER: i32 = 0;
}

pub trait HealthRecord {
    const IDENTIFIER: i32;
    fn get_identifier(&self) -> i32;
}

impl HealthRecord for StepsRecord {
    const IDENTIFIER: i32 = StepsRecord::IDENTIFIER;
    fn get_identifier(&self) -> i32 {
        Self::IDENTIFIER
    }
}