use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StepsRecord {
    // #[n(0)]
    pub startTime: String,
    // #[n(1)]
    pub endTime: String,
    // #[n(2)]
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