use crate::handlers_user::filter_db_record;
use crate::model::AppUserModel;
use crate::{handlers_ota, handlers_ra, handlers_user, AppState};
use actix_identity::{Identity, IdentityMiddleware};
use actix_web::cookie::Key;
use actix_web::dev::ServiceRequest;
use actix_web::{get, Error, HttpResponse, Responder};
use actix_web::{web, HttpMessage};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use futures::TryFutureExt;
use sha2::{Digest, Sha512};

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let app_state = match req.app_data::<web::Data<AppState>>() {
        Some(app_state) => app_state,
        None => {
            return Err((
                actix_web::error::ErrorInternalServerError("AppState is missing!"),
                req,
            ))
        }
    };

    let user = match sqlx::query_as!(
        AppUserModel,
        "SELECT * FROM app_user WHERE username = $1",
        credentials.user_id()
    )
    .fetch_optional(&app_state.get_ref().db)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                actix_web::error::ErrorUnauthorized(
                    serde_json::json!({"status": "error", "message": "Bad credentials"}),
                ),
                req,
            ))
        }
        Err(_) => {
            return Err((
                actix_web::error::ErrorUnauthorized(
                    serde_json::json!({"status": "error", "message": "Bad credentials"}),
                ),
                req,
            ))
        }
    };

    let password = credentials.password().unwrap_or_default();
    let mut hasher = Sha512::new();
    hasher.update(password);
    let password_hash = hex::encode(hasher.finalize().to_vec());

    if password_hash != user.password_hash {
        return Err((
            actix_web::error::ErrorUnauthorized(
                serde_json::json!({"status": "error", "message": "Bad credentials"}),
            ),
            req,
        ));
    }
    Identity::login(&req.extensions(), user.code.to_owned()).unwrap();

    Ok(req)
}
#[get("/user-info")]
pub async fn get_user_info(
    identity: Option<Identity>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = identity.unwrap().id().unwrap();

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
pub fn config(conf: &mut web::ServiceConfig) {
    let secret_key = Key::generate();

    let scope = web::scope("/svc")
        .wrap(HttpAuthentication::basic(validator))
        .wrap(IdentityMiddleware::default())
        .service(get_user_info)
        .service(handlers_user::get_app_user)
        .service(handlers_user::get_app_user_list)
        .service(handlers_user::post_app_user)
        .service(handlers_user::patch_app_user)
        .service(handlers_ota::get_one_time_activity_list)
        .service(handlers_ota::get_one_time_activity)
        .service(handlers_ota::post_one_time_activity)
        .service(handlers_ota::patch_one_time_activity)
        .service(handlers_ra::get_repeated_activity_list)
        .service(handlers_ra::get_repeated_activity)
        .service(handlers_ra::post_repeated_activity)
        .service(handlers_ra::patch_repeated_activity);

    conf.service(scope);
}
