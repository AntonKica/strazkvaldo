use crate::model::{AppUserModel, AppUserModelResponse};
use crate::schema::{CreateAppUser, FilterOptions, UpdateAppUser};
use crate::AppState;
use actix_web::http;
use actix_web::http::header::*;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use chrono::Utc;
use sha2::{Digest, Sha512};

pub fn filter_db_record(app_user_model: &AppUserModel) -> AppUserModelResponse {
    AppUserModelResponse {
        code: app_user_model.code.to_owned(),
        first_name: app_user_model.first_name.to_owned(),
        last_name: app_user_model.last_name.to_owned(),
        email: app_user_model.email.to_owned(),
        username: app_user_model.username.to_owned(),
        app_user_role: app_user_model.app_user_role.to_owned() as i32,
        created: app_user_model
            .created
            .format("%d.%m.%Y %H:%M:%S")
            .to_string()
            .to_owned(),
        updated: app_user_model
            .updated
            .format("%d.%m.%Y %H:%M:%S")
            .to_string()
            .to_owned(),
    }
}

#[get("/app-user")]
pub async fn get_app_user_list(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let app_user_models: Vec<AppUserModel> = sqlx::query_as!(
        AppUserModel,
        r#"SELECT * FROM app_user LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let app_users_response = app_user_models
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<AppUserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results":app_users_response.len(),
        "app_users":app_users_response
    });
    HttpResponse::Ok().json(json_response)
}
#[get("/app-user/{code}")]
pub async fn get_app_user(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner();
    match sqlx::query_as!(AppUserModel, "SELECT * FROM app_user WHERE code = $1", code)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "user": filter_db_record(&user), })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("User with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

#[post("/app-user")]
pub async fn post_app_user(
    body: web::Json<CreateAppUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let top_code: String =
        sqlx::query_scalar("SELECT code FROM app_user ORDER BY code DESC LIMIT 1")
            .fetch_one(&data.db)
            .await
            .unwrap_or("USR-0000".to_owned());

    let top_code_number = top_code
        .strip_prefix("USR-")
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();
    let next_code = format!("USR-{:04}", top_code_number + 1);

    let query_result = sqlx::query_as!(
        AppUserModel,
        r#"INSERT INTO app_user (code,first_name,last_name,email,username,password_hash,app_user_role,created,updated) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning *"#,
        next_code,
        body.first_name.to_owned(),
        body.last_name.to_owned(),
        body.email.to_owned(),
        body.username.to_owned(),
        body.password.to_owned(),
        body.app_user_role.to_owned(),
        Utc::now(),
        Utc::now(),
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            let note_response =
                serde_json::json!({"status": "success", "message": "User was created"});
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

#[patch("/app-user/{code}")]
pub async fn patch_app_user(
    path: web::Path<String>,
    body: web::Json<UpdateAppUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result =
        sqlx::query_as!(AppUserModel, "SELECT * FROM app_user WHERE code = $1", code)
            .fetch_one(&data.db)
            .await;

    if query_result.is_err() {
        let message = format!("Repeated activity with code: {} not found", code);
        return HttpResponse::Ok().json(serde_json::json!({"status": "fail","message": message}));
    }

    let app_user = query_result.unwrap();

    //let mut hasher = Sha512::new();
    //hasher.update(password);
    //let password_hash = ;
    let password_hash = match body.password.to_owned() {
        Some(password) => {
            let mut hasher = Sha512::new();
            hasher.update(password);

            hex::encode(hasher.finalize().to_vec())
        }
        None => app_user.password_hash.to_owned(),
    };

    //let now = Utc::now();
    //let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        AppUserModel,
        "UPDATE app_user SET first_name = $2, last_name = $3, email = $4, username = $5, password_hash = $6, app_user_role = $7, updated = $8 WHERE code = $1 RETURNING *",
        code,
        body.first_name.to_owned(),
        body.last_name.to_owned(),
        body.email.to_owned(),
        body.username.to_owned(),
        password_hash,
        body.app_user_role.to_owned(),
        Utc::now(),
    ) .fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(app_user) => {
            let response_body = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_db_record(&app_user)
            })});

            return HttpResponse::Ok().json(response_body);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}
