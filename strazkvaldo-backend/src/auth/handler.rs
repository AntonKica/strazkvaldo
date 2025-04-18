use crate::handlers_user::filter_db_record;
use crate::model::AppUserModel;
use crate::AppState;
use actix_session::Session;
use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use futures::TryFutureExt;
use sha2::{Digest, Sha512};

#[get("/user-info")]
async fn get_user_info(session: Session, data: web::Data<AppState>) -> impl Responder {
    let code = session.get::<String>("code").unwrap().unwrap();

    match sqlx::query_as!(
        AppUserModel,
        "SELECT * FROM app_user WHERE code = $1",
        code.clone()
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "user": filter_db_record(&user), })});
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let message = format!("User with ID: {} not found", code);
            HttpResponse::Ok().json(serde_json::json!({"status": "failure","message": message}))
        }
    }
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    username: String,
    password: String,
}
#[actix_web::post("/login")]
async fn login_user(
    data: web::Data<AppState>,
    login_data: actix_web::web::Json<LoginUser>,
    session: actix_session::Session,
) -> HttpResponse {
    let user = match sqlx::query_as!(
        AppUserModel,
        "SELECT * FROM app_user WHERE username = $1",
        login_data.username.as_str()
    )
    .fetch_optional(&data.db)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error", "message": "Bad credentials"}));
        }
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error", "message": "Bad credentials"}));
        }
    };

    let mut hasher = Sha512::new();
    hasher.update(login_data.password.as_bytes());
    let password_hash = hex::encode(hasher.finalize().to_vec());

    if password_hash != user.password_hash {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "Bad credentials"}));
    }
    session.renew();
    session
        .insert("code", user.code.clone())
        .expect("Unable to renew username");
    session
        .insert("role", user.app_user_role.clone())
        .expect("Unable to renew password");

    HttpResponse::Ok().json(
        serde_json::json!({"status": "success", "code": user.code.to_owned(), "role": user.app_user_role.to_owned() as i32}),
    )
}
#[actix_web::post("/logout")]
async fn log_out(session: actix_session::Session) -> actix_web::HttpResponse {
    match session.get::<String>("code") {
        Ok(_) => {
            session.purge();
            HttpResponse::Ok().json(serde_json::json!({ "message": "You have successfully logged out"}))
        }
        Err(e) => {
            HttpResponse::BadRequest()  .json(serde_json::json!({ "message": "We currently have some issues. Kindly try again and ensure you are logged in"}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/auth")
            .service(login_user)
            .service(log_out)
            .service(get_user_info),
    );
}
