use crate::model::{OneTimeActivityModel, OneTimeActivityModelResponse};
use crate::schema::{CreateOneTimeActivity, FilterOptions, UpdateOneTimeActivity};
use crate::AppState;
use actix_web::http;
use actix_web::http::header::*;
use actix_web::web::service;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};

fn filter_db_record(one_time_acivity_model: &OneTimeActivityModel) -> OneTimeActivityModelResponse {
    OneTimeActivityModelResponse {
        code: one_time_acivity_model.code.to_owned(),
        name: one_time_acivity_model.name.to_owned(),
        activity_type: one_time_acivity_model.activity_type.to_string().to_owned(),
        criticality_type: one_time_acivity_model
            .criticality_type
            .to_string()
            .to_owned(),
        duration_in_seconds: one_time_acivity_model.duration_in_seconds.to_owned(),
        description: one_time_acivity_model.description.to_owned(),
        date: one_time_acivity_model
            .date
            .format("%d.%m.%Y %H:%M:%S")
            .to_string()
            .to_owned(),
    }
}

#[get("/one-time-activity")]
async fn get_one_time_activity_list(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<OneTimeActivityModel> =
        sqlx::query_as!(OneTimeActivityModel, r#"SELECT * FROM one_time_activity"#)
            .fetch_all(&data.db)
            .await
            .unwrap();

    let one_time_activities_response = one_time_activities
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<OneTimeActivityModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":one_time_activities_response.len(),
        "one_time_activities":one_time_activities_response
    });
    HttpResponse::Ok().json(json_response)
}
#[get("/one-time-activity/{code}")]
async fn get_one_time_activity(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = path.into_inner();
    match sqlx::query_as!(
        OneTimeActivityModel,
        "SELECT * FROM one_time_activity WHERE code = $1",
        code
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(one_time_activity) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "one_time_activity": one_time_activity, })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("One time activity with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

#[post("/one-time-activity")]
async fn post_one_time_activity_list(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<OneTimeActivityModel> =
        sqlx::query_as!(OneTimeActivityModel, r#"SELECT * FROM one_time_activity"#)
            .fetch_all(&data.db)
            .await
            .unwrap();

    let one_time_activities_response = one_time_activities
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<OneTimeActivityModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":one_time_activities_response.len(),
        "one_time_activities":one_time_activities_response
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/one-time-activity")]
async fn post_one_time_activity(
    body: web::Json<CreateOneTimeActivity>,
    data: web::Data<AppState>,
) -> impl Responder {
    let top_code: String =
        sqlx::query_scalar("SELECT code FROM one_time_activity ORDER BY code DESC LIMIT 1")
            .fetch_one(&data.db)
            .await
            .unwrap_or("OTA-0000".to_owned());

    let top_code_number = top_code
        .strip_prefix("OTA-")
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();
    let next_code = format!("OTA-{:04}", top_code_number + 1);

    let query_result = sqlx::query_as!(
        OneTimeActivityModel,
        r#"INSERT INTO one_time_activity (code,name,activity_type,criticality_type,duration_in_seconds,description,date) VALUES ($1,$2,$3,$4,$5,$6,$7) returning *"#,
        next_code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.date.to_owned(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(_) => {
            let note_response =
                serde_json::json!({"status": "success", "message": "Activity was created"});
            return HttpResponse::Created()
                .append_header((
                    http::header::LOCATION,
                    HeaderValue::from_str(next_code.as_str()).unwrap(),
                ))
                .json(note_response);
        }
        Err(e) => {
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[patch("/one-time-activity/{code}")]
async fn patch_one_time_activity(
    path: web::Path<String>,
    body: web::Json<UpdateOneTimeActivity>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(
        OneTimeActivityModel,
        "SELECT * FROM one_time_activity WHERE code = $1",
        code
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("One time activity with code: {} not found", code);
        return HttpResponse::Ok().json(serde_json::json!({"status": "fail","message": message}));
    }

    //let now = Utc::now();
    //let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        OneTimeActivityModel,
        "UPDATE one_time_activity SET name = $2, activity_type = $3, criticality_type = $4, duration_in_seconds = $5, description = $6, date =$7 WHERE code = $1 RETURNING *",
        code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.date.to_owned(),
    ) .fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "one_time_activity": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/svc")
        .service(get_one_time_activity_list)
        .service(get_one_time_activity)
        .service(post_one_time_activity)
        .service(patch_one_time_activity);

    conf.service(scope);
}
