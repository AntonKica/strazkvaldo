use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOneTimeActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub description: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateOneTimeActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub date: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRepeatedActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
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
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub description: String,
    pub periodicity: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAppUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub app_user_role: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAppUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub app_user_role: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoom {
    pub name: String,
    pub room_type: String,
    pub description: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoom {
    pub name: String,
    pub room_type: String,
    pub description: String,
}
