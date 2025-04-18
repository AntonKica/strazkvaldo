mod auth;
mod enums;
mod handlers;
mod handlers_ota;
mod handlers_ra;
mod handlers_user;
mod model;
mod schema;

use actix_web::middleware::from_fn;
use actix_web::web::scope;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::cmp::PartialEq;

pub struct AppState {
    db: PgPool,
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

    let secret_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
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
