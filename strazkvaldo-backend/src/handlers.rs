use crate::{handlers_ota, handlers_ra};
use actix_web::web;
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/svc")
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
