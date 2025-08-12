use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingData {
    pub features: Vec<Vec<f32>>,
    pub labels: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub loss: Option<f32>,
    pub accuracy: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUpdate {
    pub weights: Vec<f32>,
    pub metrics: Option<TrainingMetrics>,
}