use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct OneTimeActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct OneTimeActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: String,
}
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RepeatedActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RepeatedActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: String,
    pub start_date: String,
    pub end_date: String,
}
