use crate::enums::AppUserRole;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AppUserModel {
    pub code: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub app_user_role: AppUserRole,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AppUserModelResponse {
    pub code: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub app_user_role: i32,
    pub created: String,
    pub updated: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: i32,
    pub criticality_type: i32,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: String,
}
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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
