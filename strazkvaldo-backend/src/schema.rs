use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOneTimeActivity {
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub description: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateOneTimeActivity {
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRepeatedActivity {
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub description: String,
    pub periodicity: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRepeatedActivity {
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
