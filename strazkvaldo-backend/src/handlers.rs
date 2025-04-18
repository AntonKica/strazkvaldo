use crate::{handlers_ota, handlers_ra, handlers_user};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
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
