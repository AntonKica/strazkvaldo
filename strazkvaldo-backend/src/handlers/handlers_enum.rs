use crate::application_enums::{AppUserRole, CriticalityType, EnumModelResponseTrait, Periodicity};
use crate::model::{EnumModel, EnumModelResponse};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};
use std::str::FromStr;

pub enum EnumType {
    RoomType,
    ActivityType,
}

fn enum_string_to_enum_type(enum_string: &String) -> Option<EnumType> {
    match enum_string.as_str() {
        "room-type" => Some(EnumType::RoomType),
        "activity-type" => Some(EnumType::ActivityType),
        _ => None,
    }
}
fn enum_type_to_db_column(enum_type: &EnumType) -> String {
    match enum_type {
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
    if enum_string_to_enum_type(&name).is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "failed", "message": "Invalid enum name"}));
    }

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

pub fn get_enum_for_application_enum<T: FromStr + ToString + EnumModelResponseTrait>(
    code: String,
) -> Option<EnumModelResponse> {
    match T::from_str(code.as_str()).map(|v| EnumModelResponse {
        code: v.to_string(),
        text: v.to_text().to_string(),
    }) {
        Ok(enum_model) => Some(enum_model),
        Err(_) => None,
    }
}
#[get("/enum/app-user-role")]
pub async fn get_app_user_role(data: web::Data<AppState>) -> impl Responder {
    application_enum_crete_response(vec![AppUserRole::Admin, AppUserRole::User])
}
#[get("/enum/criticality-type")]
pub async fn get_criticality_type(data: web::Data<AppState>) -> impl Responder {
    application_enum_crete_response(vec![
        CriticalityType::Low,
        CriticalityType::Medium,
        CriticalityType::High,
    ])
}
#[get("/enum/periodicity")]
pub async fn get_periodicity(data: web::Data<AppState>) -> impl Responder {
    application_enum_crete_response(vec![
        Periodicity::Day,
        Periodicity::Week,
        Periodicity::Month,
        Periodicity::Year,
    ])
}

fn application_enum_crete_response<T: ToString + EnumModelResponseTrait>(
    e: Vec<T>,
) -> HttpResponse {
    let enum_values = e
        .iter()
        .map(|v| EnumModelResponse {
            code: v.to_string(),
            text: v.to_text().to_string(),
        })
        .collect::<Vec<EnumModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":enum_values.len(),
        "enum_values":enum_values
    });
    HttpResponse::Ok().json(json_response)
}
