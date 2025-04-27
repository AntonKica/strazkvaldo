use crate::application_enums::{CriticalityType, Periodicity};
use crate::handlers::handlers_enum::{get_enum_for, get_enum_for_application_enum, EnumType};
use crate::model::{RepeatedActivityModel, RepeatedActivityModelResponse};
use crate::schema::{CreateRepeatedActivity, FilterOptions, UpdateRepeatedActivity};
use crate::AppState;
use actix_web::http;
use actix_web::http::header::*;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use std::str::FromStr;
use std::sync::Arc;

fn filter_db_record(
    repeated_activity_model: &RepeatedActivityModel,
    data: &web::Data<Arc<AppState>>,
) -> RepeatedActivityModelResponse {
    RepeatedActivityModelResponse {
        code: repeated_activity_model.code.to_owned(),
        name: repeated_activity_model.name.to_owned(),
        activity_type: get_enum_for(
            EnumType::ActivityType,
            repeated_activity_model.activity_type.to_owned(),
            data.clone(),
        )
        .unwrap(),
        criticality_type: get_enum_for_application_enum::<CriticalityType>(
            repeated_activity_model.criticality_type.clone(),
        )
        .unwrap(),
        duration_in_seconds: repeated_activity_model.duration_in_seconds.to_owned(),
        periodicity: get_enum_for_application_enum::<Periodicity>(
            repeated_activity_model.periodicity.clone(),
        )
        .unwrap(),
        description: repeated_activity_model.description.to_owned(),
        periodicity_unit: repeated_activity_model.periodicity_unit,
    }
}

#[get("/repeated-activity")]
pub async fn get_repeated_activity_list(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let repeated_activities: Vec<RepeatedActivityModel> = sqlx::query_as!(
        RepeatedActivityModel,
        r#"SELECT * FROM repeated_activity LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let repeated_activities_response = repeated_activities
        .into_iter()
        .map(|note| filter_db_record(&note, &data))
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
    data: web::Data<Arc<AppState>>,
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
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "repeated_activity": filter_db_record(&repeated_activity, &data), })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Repeated activity with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

fn check_periodicity(periodicity: &Periodicity, periodicity_unit: i32) -> Result<(), HttpResponse> {
    match periodicity {
        Periodicity::Day => Ok(()),
        Periodicity::Week => {
            if periodicity_unit >= 1 && periodicity_unit <= 7 {
                Ok(())
            } else {
                Err(HttpResponse::BadRequest().json(serde_json::json!({"staus": "failed", "message": "day if week is out of range"})))
            }
        }
        Periodicity::Month => {
            if periodicity_unit >= 1 && periodicity_unit <= 12 {
                Ok(())
            } else {
                Err(HttpResponse::BadRequest().json(serde_json::json!({"staus": "failed", "message": "month of year is out of range"})))
            }
        }
        Periodicity::Year => {
            if periodicity_unit >= 1 && periodicity_unit <= 366 {
                Ok(())
            } else {
                Err(HttpResponse::BadRequest().json(serde_json::json!({"staus": "failed", "message": "day of year is out of range"})))
            }
        }
    }
}

#[post("/repeated-activity")]
pub async fn post_repeated_activity(
    body: web::Json<CreateRepeatedActivity>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    if let Err(error) = check_periodicity(
        &Periodicity::from_str(body.periodicity.as_str()).unwrap(),
        body.periodicity_unit,
    ) {
        return error;
    }

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
        r#"INSERT INTO repeated_activity (code,name,activity_type,criticality_type,duration_in_seconds,description,periodicity,periodicity_unit) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) returning *"#,
        next_code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.periodicity.to_owned(),
        body.periodicity_unit.to_owned()
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
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    if let Err(error) = check_periodicity(
        &Periodicity::from_str(body.periodicity.as_str()).unwrap(),
        body.periodicity_unit,
    ) {
        return error;
    }

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
        "UPDATE repeated_activity SET name = $2, activity_type = $3, criticality_type = $4, duration_in_seconds = $5, description = $6, periodicity = $7, periodicity_unit = $8 WHERE code = $1 RETURNING *",
        code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.description.to_owned(),
        body.periodicity.to_owned(),
        body.periodicity_unit.to_owned(),
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
