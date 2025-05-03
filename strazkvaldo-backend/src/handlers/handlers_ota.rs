use crate::application_enums::CriticalityType;
use crate::handlers::handlers_enum::{get_enum_for, get_enum_for_application_enum, EnumType};
use crate::handlers::handlers_room::get_simple_room;
use crate::model::{OneTimeActivityModel, OneTimeActivityModelResponse};
use crate::schema::{CreateOneTimeActivity, FilterOptions, UpdateOneTimeActivity};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::{delete, http};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use std::sync::Arc;

async fn filter_db_record(
    one_time_activity_model: &OneTimeActivityModel,
    data: &web::Data<Arc<AppState>>,
) -> OneTimeActivityModelResponse {
    OneTimeActivityModelResponse {
        code: one_time_activity_model.code.to_owned(),
        name: one_time_activity_model.name.to_owned(),
        activity_type: get_enum_for(
            EnumType::ActivityType,
            one_time_activity_model.activity_type.to_owned(),
            data.clone(),
        )
        .unwrap(),
        criticality_type: get_enum_for_application_enum::<CriticalityType>(
            one_time_activity_model.criticality_type.clone(),
        )
        .unwrap(),
        duration_in_seconds: one_time_activity_model.duration_in_seconds.to_owned(),
        room: get_simple_room(one_time_activity_model.room_code.clone(), &data.db).await,
        description: one_time_activity_model.description.to_owned(),
        due_date: one_time_activity_model.due_date,
    }
}

#[get("/one-time-activity")]
pub async fn get_one_time_activity_list(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(100);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<OneTimeActivityModel> = sqlx::query_as!(
        OneTimeActivityModel,
        r#"SELECT * FROM one_time_activity where _removed = false and due_date >= CURRENT_DATE order by due_date desc LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let one_time_activities_expired: Vec<OneTimeActivityModel> = sqlx::query_as!(
        OneTimeActivityModel,
        r#"SELECT * FROM one_time_activity where _removed = false and due_date < CURRENT_DATE order by due_date LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let one_time_activities_response = futures::future::join_all(
        one_time_activities
            .into_iter()
            .map(async |note| filter_db_record(&note, &data).await),
    )
    .await;

    let one_time_activities_expired_response = futures::future::join_all(
        one_time_activities_expired
            .into_iter()
            .map(async |note| filter_db_record(&note, &data).await),
    )
    .await;

    let json_response = serde_json::json!({
        "status": "success",
        "results":one_time_activities_response.len(),
        "one_time_activities":one_time_activities_response,
        "one_time_activities_expired":one_time_activities_expired_response
    });
    HttpResponse::Ok().json(json_response)
}
#[get("/one-time-activity/{code}")]
pub async fn get_one_time_activity(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
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
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "one_time_activity": filter_db_record(&one_time_activity, &data).await, })});
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let message = format!("One time activity with ID: {} not found", code);
            HttpResponse::Ok().json(serde_json::json!({"status": "failure","message": message}))
        }
    }
}
#[post("/one-time-activity")]
pub async fn post_one_time_activity(
    body: web::Json<CreateOneTimeActivity>,
    data: web::Data<Arc<AppState>>,
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
        r#"INSERT INTO one_time_activity (code,name,activity_type,criticality_type,duration_in_seconds,room_code,description,due_date,_removed) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,false) returning *"#,
        next_code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.room_code.to_owned(),
        body.description.to_owned(),
        body.due_date.to_owned(),
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
pub async fn patch_one_time_activity(
    path: web::Path<String>,
    body: web::Json<UpdateOneTimeActivity>,
    data: web::Data<Arc<AppState>>,
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
        "UPDATE one_time_activity SET name = $2, activity_type = $3, criticality_type = $4, duration_in_seconds = $5, room_code = $6, description = $7, due_date =$8 WHERE code = $1 RETURNING *",
        code,
        body.name.to_owned(),
        body.activity_type.to_owned(),
        body.criticality_type.to_owned(),
        body.duration_in_seconds.to_owned(),
        body.room_code.to_owned(),
        body.description.to_owned(),
        body.due_date.to_owned(),
    ).fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "one_time_activity": note
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}))
        }
    }
}

#[delete("/one-time-activity/{code}")]
pub async fn delete_one_time_activity(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(
        OneTimeActivityModel,
        "UPDATE one_time_activity SET _removed = true WHERE code = $1 RETURNING *",
        code,
    )
    .fetch_one(&data.db)
    .await;
    match query_result {
        Ok(note) => {
            let note_response =
                serde_json::json!({"status": "success","message": "successfully removed"});
            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("failed to remove: {:?}", err);
            HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}))
        }
    }
}
