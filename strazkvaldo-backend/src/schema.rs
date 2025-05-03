use chrono::NaiveDate;
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
    pub room_code: String,
    pub description: String,
    pub due_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateOneTimeActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub room_code: String,
    pub description: String,
    pub due_date: NaiveDate,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRepeatedActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub room_code: String,
    pub description: String,
    pub periodicity: String,
    pub periodicity_unit: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRepeatedActivity {
    pub name: String,
    pub activity_type: String,
    pub criticality_type: String,
    pub duration_in_seconds: i32,
    pub room_code: String,
    pub description: String,
    pub periodicity: String,
    pub periodicity_unit: i32,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct FinishedActivityModel {
    pub code: String,
    pub repeated_activity_code: Option<String>,
    pub one_time_activity_code: Option<String>,
    pub due_date: NaiveDate,
    pub description: String,
    pub reviewed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReviewFinishedActivityModel {
    pub description: String,
}
