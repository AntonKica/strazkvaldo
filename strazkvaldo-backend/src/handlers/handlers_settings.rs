use crate::model::{AppSettings, AppSettingsModel};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::{get, patch, web, HttpResponse, Responder};
use sha2::Digest;
use sqlx::{Error, PgPool};
use std::sync::Arc;

pub async fn query_app_settings(db: &PgPool) -> Result<AppSettings, Error> {
    sqlx::query_as!(AppSettingsModel, r#"SELECT * FROM app_settings"#,)
        .fetch_one(db)
        .await
        .map(|app_settings_model| AppSettings {
            auto_review_finished_activity: app_settings_model.auto_review_finished_activity,
            auto_review_finished_activity_days: app_settings_model
                .auto_review_finished_activity_days,
        })
}
#[get("/app-settings")]
pub async fn get_app_settings(data: web::Data<Arc<AppState>>) -> impl Responder {
    match query_app_settings(&data.db).await
    {
        Ok(settings) => {
            let response = serde_json::json!({"status": "success", "data" : {"app_settings": settings}});
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": "Failed to get settings", "error": err.to_string()}))
        }
    }
}

#[patch("/app-settings")]
pub async fn patch_app_settings(
    body: web::Json<AppSettings>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        AppSettingsModel,
        "UPDATE app_settings SET auto_review_finished_activity = $1, auto_review_finished_activity_days = $2 RETURNING *",
        body.auto_review_finished_activity,
        body.auto_review_finished_activity_days,
    ) .fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(app_user) => {
            let response_body = serde_json::json!({"status": "success"});

            HttpResponse::Ok().json(response_body)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}))
        }
    }
}
