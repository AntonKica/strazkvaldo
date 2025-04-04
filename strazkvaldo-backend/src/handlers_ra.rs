use crate::model::{RepeatedActivityModel, RepeatedActivityModelResponse};
use crate::schema::{CreateRepeatedActivity, FilterOptions, UpdateRepeatedActivity};
use crate::AppState;
use actix_web::http;
use actix_web::http::header::*;
use actix_web::web::service;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};

fn filter_db_record(
    repeated_activity_model: &RepeatedActivityModel,
) -> RepeatedActivityModelResponse {
    RepeatedActivityModelResponse {
        code: repeated_activity_model.code.to_owned(),
        name: repeated_activity_model.name.to_owned(),
        activity_type: repeated_activity_model.activity_type.to_string().to_owned(),
        criticality_type: repeated_activity_model
            .criticality_type
            .to_string()
            .to_owned(),
        duration_in_seconds: repeated_activity_model.duration_in_seconds.to_owned(),
        description: repeated_activity_model.description.to_owned(),
        periodicity: repeated_activity_model.periodicity.to_owned(),
        start_date: repeated_activity_model
            .start_date
            .format("%d.%m.%Y %H:%M:%S")
            .to_string()
            .to_owned(),
        end_date: repeated_activity_model
            .end_date
            .format("%d.%m.%Y %H:%M:%S")
            .to_string()
            .to_owned(),
    }
}

#[get("/repeated-activity")]
pub async fn get_repeated_activity_list(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let repeated_activities: Vec<RepeatedActivityModel> =
        sqlx::query_as!(RepeatedActivityModel, r#"SELECT * FROM repeated_activity"#)
            .fetch_all(&data.db)
            .await
            .unwrap();

    let repeated_activities_response = repeated_activities
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<RepeatedActivityModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":repeated_activities_response.len(),
        "repeated_activities":repeated_activities_response
    });
    HttpResponse::Ok().json(json_response)
}
#[get("/repeated-activity/{code}")]
pub async fn get_repeated_activity(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = path.into_inner();
    match sqlx::query_as!(
        RepeatedActivityModel,
        "SELECT * FROM repeated_activity WHERE code = $1",
        code
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(repeated_activity) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "repeated_activity": repeated_activity, })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Repeated activity with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

#[post("/repeated-activity")]
pub async fn post_repeated_activity_list(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let repeated_activities: Vec<RepeatedActivityModel> =
        sqlx::query_as!(RepeatedActivityModel, r#"SELECT * FROM repeated_activity"#)
            .fetch_all(&data.db)
            .await
            .unwrap();

    let repeated_activities_response = repeated_activities
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<RepeatedActivityModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":repeated_activities_response.len(),
        "repeated_activities":repeated_activities_response
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/repeated-activity")]
pub async fn post_repeated_activity(
    body: web::Json<CreateRepeatedActivity>,
    data: web::Data<AppState>,
) -> impl Responder {
    let top_code: String =
        sqlx::query_scalar("SELECT code FROM repeated_activity ORDER BY code DESC LIMIT 1")
            .fetch_one(&data.db)
            .await
            .unwrap_or("RA-0000".to_owned());

    let top_code_number = top_code
        .strip_prefix("RA-")
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();
    let next_code = format!("RA-{:04}", top_code_number + 1);

    let query_result = sqlx::query_as!(
        RepeatedActivityModel,
        r#"INSERT INTO repeated_activity (code,name,activity_type,criticality_type,duration_in_seconds,description,periodicity,start_date,end_date) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning *"#,
        next_code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.periodicity.to_owned(),
        body.start_date.to_owned(),
        body.end_date.to_owned(),
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

#[patch("/repeated-activity/{code}")]
pub async fn patch_repeated_activity(
    path: web::Path<String>,
    body: web::Json<UpdateRepeatedActivity>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(
        RepeatedActivityModel,
        "SELECT * FROM repeated_activity WHERE code = $1",
        code
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Repeated activity with code: {} not found", code);
        return HttpResponse::Ok().json(serde_json::json!({"status": "fail","message": message}));
    }

    //let now = Utc::now();
    //let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        RepeatedActivityModel,
        "UPDATE repeated_activity SET name = $2, activity_type = $3, criticality_type = $4, duration_in_seconds = $5, description = $6, periodicity = $7, start_date = $8, end_date = $9 WHERE code = $1 RETURNING *",
        code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.periodicity.to_owned(),
        body.start_date.to_owned(),
        body.end_date.to_owned(),
    ) .fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "repeated_activity": note
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
