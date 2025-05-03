mod application_enums;
mod auth;
mod handlers;
mod model;
mod schema;
mod tests;

use crate::handlers::handlers_activities::{
    auto_review_finished_activities_for_today, generate_finished_activities_for_today,
};
use crate::model::EnumModel;
use actix_web::middleware::from_fn;
use actix_web::web::scope;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::cmp::PartialEq;
use std::ops::Deref;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct AppState {
    db: PgPool,
    enum_values: Vec<EnumModel>,
}

#[derive(Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
    pub fn from_str(s: &str) -> Option<Environment> {
        match s {
            "prod" => Some(Environment::Production),
            "dev" => Some(Environment::Development),
            _ => None,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let environment_var = std::env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");
    let environment = Environment::from_str(environment_var.as_str()).expect("Unknown ENVIRONMENT");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let enum_values = sqlx::query_as!(EnumModel, r#"SELECT * FROM enum_values"#)
        .fetch_all(&pool)
        .await
        .unwrap();

    let secret_key = actix_web::cookie::Key::generate();

    let app_data = web::Data::new(AppState {
        db: pool.clone(),
        enum_values: enum_values.clone(),
    });
    let cron_data = Arc::clone(&app_data);

    let sched = JobScheduler::new().await.unwrap();

    // Add cron job with access to the data
    let job = Job::new("0 0 0 * * *", move |_uuid, _l| {
        let data = Arc::clone(&cron_data);
        tokio::spawn(async move {
            data.db.acquire().await.unwrap();
            generate_finished_activities_for_today(&data.db).await;
            auto_review_finished_activities_for_today(&data.db).await;
        });
    })
    .unwrap();
    sched.add(job).await.unwrap();
    sched.start().await.unwrap();

    // run on startup
    generate_finished_activities_for_today(&pool).await;
    auto_review_finished_activities_for_today(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_data)))
            .wrap(from_fn(auth::middleware::auth_middleware))
            .wrap(match environment {
                Environment::Development => actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(true)
                .build(),
                Environment::Production => actix_session::SessionMiddleware::new(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                ),
            })
            .wrap(middleware::Logger::default())
            .service(
                scope("/svc")
                    .configure(auth::handler::config)
                    .configure(handlers::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
