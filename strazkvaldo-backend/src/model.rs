use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BitVec;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AppUserModel {
    pub code: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub app_user_role: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub _removed: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AppUserModelResponse {
    pub code: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub app_user_role: EnumModelResponse,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub due_date: NaiveDate,
    pub _removed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: EnumModelResponse,
    pub criticality_type: EnumModelResponse,
    pub duration_in_seconds: i32,
    pub description: String,
    pub due_date: NaiveDate,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RepeatedActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: String,
    pub periodicity_unit: i32,
    pub _removed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepeatedActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: EnumModelResponse,
    pub criticality_type: EnumModelResponse,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: EnumModelResponse,
    pub periodicity_unit: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomModel {
    pub code: String,
    pub name: String,
    pub room_type: String,
    pub description: String,
    pub _removed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomModelResponse {
    pub code: String,
    pub name: String,
    pub room_type: EnumModelResponse,
    pub description: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumModel {
    pub name: String,
    pub code: String,
    pub text: String,
    pub _removed: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumModelResponse {
    pub code: String,
    pub text: String,
}

#[derive(Debug)]
pub struct UpcomingActivity {
    pub name: String,
    pub repeated_activity_code: Option<String>,
    pub one_time_activity_code: Option<String>,
    pub due_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct UpcomingActivityResponse {
    pub name: String,
    pub repeated_activity_code: Option<String>,
    pub one_time_activity_code: Option<String>,
    pub due_date: String,
}

#[derive(Debug)]
pub struct FinishedActivity {
    pub name: String,
    pub repeated_activity_code: String,
    pub one_time_activity_code: String,
    pub due_date: NaiveDate,
    pub description: String,
    pub reviewed: bool,
}

#[derive(Debug, Serialize)]
pub struct FinishedActivityResponse {
    pub code: String,
    pub repeated_activity_code: Option<String>,
    pub one_time_activity_code: Option<String>,
    pub due_date: NaiveDate,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AppSettingsModel {
    pub _lock: BitVec,
    pub auto_review_finished_activity: bool,
    pub auto_review_finished_activity_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_review_finished_activity: bool,
    pub auto_review_finished_activity_days: i32,
}
