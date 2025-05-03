use crate::application_enums::AppUserRole;
use crate::auth;
use actix_web::middleware::from_fn;
use actix_web::web;

pub mod handlers_activities;
mod handlers_enum;
mod handlers_ota;
mod handlers_ra;
mod handlers_room;
mod handlers_settings;
mod handlers_user;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(
            web::scope("/admin")
                .wrap(from_fn(|req, srv| async move {
                    auth::middleware::check_role_middleware(req, srv, AppUserRole::Admin).await
                }))
                .service(handlers_user::get_app_user)
                .service(handlers_user::get_app_user_list)
                .service(handlers_user::post_app_user)
                .service(handlers_user::patch_app_user)
                .service(handlers_user::delete_user)
                .service(handlers_enum::post_enum),
        )
        .service(
            web::scope("/user")
                .wrap(from_fn(|req, srv| async move {
                    auth::middleware::check_role_middleware(req, srv, AppUserRole::User).await
                }))
                .service(handlers_ota::get_one_time_activity_list)
                .service(handlers_ota::get_one_time_activity)
                .service(handlers_ota::post_one_time_activity)
                .service(handlers_ota::patch_one_time_activity)
                .service(handlers_ota::delete_one_time_activity)
                .service(handlers_ra::get_repeated_activity_list)
                .service(handlers_ra::get_repeated_activity)
                .service(handlers_ra::post_repeated_activity)
                .service(handlers_ra::patch_repeated_activity)
                .service(handlers_ra::delete_repeated_activity)
                .service(handlers_room::get_room_list)
                .service(handlers_room::get_room)
                .service(handlers_room::post_room)
                .service(handlers_room::patch_room)
                .service(handlers_room::delete_room)
                .service(handlers_activities::get_upcoming_activities)
                .service(handlers_activities::get_recently_finished_activity_list)
                .service(handlers_activities::get_recently_finished_activity)
                .service(handlers_activities::review_recently_finished_activity)
                .service(handlers_activities::get_reviewed_finished_activity_list)
                .service(handlers_settings::get_app_settings)
                .service(handlers_settings::patch_app_settings),
        )
        .service(handlers_enum::get_app_user_role)
        .service(handlers_enum::get_criticality_type)
        .service(handlers_enum::get_periodicity)
        .service(handlers_enum::get_enum);

    conf.service(scope);
}
