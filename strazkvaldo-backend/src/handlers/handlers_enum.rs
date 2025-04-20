use crate::model::{EnumModel, EnumModelResponse};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};

pub enum EnumType {
    AppUserRole,
    CriticalityType,
    RoomType,
    ActivityType,
}

fn enum_type_to_db_column(enum_type: &EnumType) -> String {
    match enum_type {
        EnumType::AppUserRole => "app-user-role",
        EnumType::CriticalityType => "criticality-type",
        EnumType::RoomType => "room-type",
        EnumType::ActivityType => "activity-type",
    }
    .to_string()
}

pub fn filter_db_record(enum_model: &EnumModel) -> EnumModelResponse {
    EnumModelResponse {
        code: enum_model.code.to_owned(),
        text: enum_model.text.to_owned(),
    }
}

pub fn get_enum_for(
    enum_type: EnumType,
    code: String,
    data: &Data<AppState>,
) -> Option<EnumModelResponse> {
    data.enum_values
        .iter()
        .find(|e| e.name == enum_type_to_db_column(&enum_type) && e.code == code)
        .map(|e| filter_db_record(&e))
}

#[get("/enum/{name}")]
pub async fn get_enum(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = path.into_inner();
    let enum_values: Vec<EnumModel> = sqlx::query_as!(
        EnumModel,
        r#"SELECT * FROM enum_values WHERE name = $1"#,
        name
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let enum_response = enum_values
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<EnumModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":enum_response.len(),
        "enum_values":enum_response
    });
    HttpResponse::Ok().json(json_response)
}
