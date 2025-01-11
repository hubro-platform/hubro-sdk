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