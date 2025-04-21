use crate::application_enums::AppUserRole;
use crate::model::AppUserModel;
use crate::AppState;
use actix_web::web;
use actix_web::HttpResponse;
use sha2::{Digest, Sha512};
use std::str::FromStr;

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
        .insert(
            "role",
            AppUserRole::from_str(user.app_user_role.as_str()).unwrap(),
        )
        .expect("Unable to renew password");

    HttpResponse::Ok().json(
        serde_json::json!({"status": "success", "code": user.code.to_owned(), "role": user.app_user_role.to_owned()}),
    )
}
#[actix_web::post("/logout")]
async fn log_out(session: actix_session::Session) -> actix_web::HttpResponse {
    match session.get::<String>("code") {
        Ok(_) => {
            session.purge();
            HttpResponse::Ok().json(serde_json::json!({ "message": "You have successfully logged out"}))
        }
        Err(_) => {
            HttpResponse::BadRequest()  .json(serde_json::json!({ "message": "We currently have some issues. Kindly try again and ensure you are logged in"}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/auth").service(login_user).service(log_out));
}
