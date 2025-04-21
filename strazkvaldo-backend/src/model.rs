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
    pub app_user_role: String,
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
    pub app_user_role: EnumModelResponse,
    pub created: String,
    pub updated: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModel {
    pub code: String,
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeActivityModelResponse {
    pub code: String,
    pub name: String,
    pub activity_type: EnumModelResponse,
    pub criticality_type: EnumModelResponse,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: String,
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
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumModelResponse {
    pub code: String,
    pub text: String,
}
