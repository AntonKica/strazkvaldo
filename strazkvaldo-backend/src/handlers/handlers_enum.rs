use crate::application_enums::{AppUserRole, CriticalityType, EnumModelResponseTrait, Periodicity};
use crate::model::{EnumModel, EnumModelResponse};
use crate::schema::EnumUpdateModel;
use crate::AppState;
use actix_web::http::header::*;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::str::FromStr;
use std::sync::Arc;

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

pub async fn get_enum_for(
    enum_type: EnumType,
    code: String,
    data: Data<Arc<AppState>>,
) -> Option<EnumModelResponse> {
    match sqlx::query_as!(
        EnumModel,
        r#"SELECT * FROM enum_values where name = $1 and code =$2;"#,
        enum_type_to_db_column(&enum_type),
        code
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(enum_value) => Some(EnumModelResponse {
            code: enum_value.code,
            text: enum_value.text,
        }),
        Err(_) => None,
    }
}

#[get("/enum/{name}")]
pub async fn get_enum(path: web::Path<String>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let name = path.into_inner();
    if enum_string_to_enum_type(&name).is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "failed", "message": "Invalid enum name"}));
    }

    let enum_values: Vec<EnumModel> = sqlx::query_as!(
        EnumModel,
        r#"SELECT * FROM enum_values WHERE _removed = false and name = $1"#,
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

#[post("/enum/{name}")]
pub async fn post_enum(
    path: web::Path<String>,
    body: web::Json<Vec<EnumUpdateModel>>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let name = path.into_inner();
    if enum_string_to_enum_type(&name).is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "failed", "message": "Invalid enum name"}));
    }

    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match sqlx::query_as!(
        enum_values,
        "UPDATE enum_values SET _removed = true WHERE name = $1",
        name
    )
    .execute(&mut *tx)
    .await
    {
        Ok(_) => {}
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    let query = r"
            INSERT INTO enum_values
             (
                name,
                code,
                text,
                _removed
            ) SELECT * FROM UNNEST(
                $1::CHAR(16)[],
                $2::CHAR(16)[],
                $3::CHAR(80)[],
                $4::BOOLEAN[]
            ) ON CONFLICT (code) DO UPDATE SET text=excluded.text, _removed = false";

    match sqlx::query(query)
        .bind(Vec::from_iter(vec![name.clone(); body.len()]))
        .bind(body.iter().map(|e| e.code.clone()).collect::<Vec<String>>())
        .bind(body.iter().map(|e| e.text.clone()).collect::<Vec<String>>())
        .bind(Vec::from_iter(vec![false; body.len()]))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(_) => match tx.commit().await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "success", })),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
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
pub async fn get_app_user_role(data: web::Data<Arc<AppState>>) -> impl Responder {
    application_enum_crete_response(vec![AppUserRole::Admin, AppUserRole::User])
}
#[get("/enum/criticality-type")]
pub async fn get_criticality_type(data: web::Data<Arc<AppState>>) -> impl Responder {
    application_enum_crete_response(vec![
        CriticalityType::Low,
        CriticalityType::Medium,
        CriticalityType::High,
    ])
}
#[get("/enum/periodicity")]
pub async fn get_periodicity(data: web::Data<Arc<AppState>>) -> impl Responder {
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
