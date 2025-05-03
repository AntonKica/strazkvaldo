use crate::handlers::handlers_enum::{get_enum_for, EnumType};
use crate::model::{RoomModel, RoomModelResponse, RoomSimpleModelResponse};
use crate::schema::{CreateRoom, FilterOptions, UpdateRoom};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::{delete, http};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use futures::future::join_all;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_simple_rooms(db: &PgPool) -> Vec<RoomSimpleModelResponse> {
    let rooms: Vec<RoomModel> =
        sqlx::query_as!(RoomModel, r#"SELECT * FROM room where _removed = false"#)
            .fetch_all(db)
            .await
            .unwrap();

    rooms
        .into_iter()
        .map(|room_model: RoomModel| RoomSimpleModelResponse {
            code: room_model.code,
            name: room_model.name,
        })
        .collect::<Vec<RoomSimpleModelResponse>>()
}
pub async fn get_simple_room(code: String, db: &PgPool) -> RoomSimpleModelResponse {
    let room: RoomModel = sqlx::query_as!(RoomModel, r#"SELECT * FROM room where code = $1"#, code)
        .fetch_one(db)
        .await
        .unwrap();

    RoomSimpleModelResponse {
        code: room.code,
        name: room.name,
    }
}
async fn filter_db_record(
    room_model: &RoomModel,
    data: &web::Data<Arc<AppState>>,
) -> RoomModelResponse {
    RoomModelResponse {
        code: room_model.code.to_owned(),
        name: room_model.name.to_owned(),
        room_type: get_enum_for(
            EnumType::RoomType,
            room_model.room_type.to_owned(),
            data.clone(),
        )
        .await
        .unwrap(),
        description: room_model.description.to_owned(),
    }
}

#[get("/room")]
pub async fn get_room_list(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(100);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let rooms: Vec<RoomModel> = sqlx::query_as!(
        RoomModel,
        r#"SELECT * FROM room where _removed = false LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let room_response = join_all(
        rooms
            .into_iter()
            .map(async |note| filter_db_record(&note, &data).await),
    )
    .await;

    let json_response = serde_json::json!({
        "status": "success",
        "results":room_response.len(),
        "rooms":room_response
    });
    HttpResponse::Ok().json(json_response)
}
#[get("/room/{code}")]
pub async fn get_room(path: web::Path<String>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let code = path.into_inner();
    match sqlx::query_as!(RoomModel, "SELECT * FROM room WHERE code = $1", code)
        .fetch_one(&data.db)
        .await
    {
        Ok(room) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "room":  filter_db_record(&room, &data).await, })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("One time activity with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

#[post("/room")]
pub async fn post_room(
    body: web::Json<CreateRoom>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let top_code: String = sqlx::query_scalar("SELECT code FROM room ORDER BY code DESC LIMIT 1")
        .fetch_one(&data.db)
        .await
        .unwrap_or("ROOM-0000".to_owned());

    let top_code_number = top_code
        .strip_prefix("ROOM-")
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();
    let next_code = format!("ROOM-{:04}", top_code_number + 1);

    let query_result = sqlx::query_as!(
        RoomModel,
        r#"INSERT INTO room (code,name,room_type,description,_removed) VALUES ($1,$2,$3,$4,false) returning *"#,
        next_code,
        body.name.to_owned(),
        body.room_type.to_owned(),
        body.description.to_owned(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(_) => {
            let note_response =
                serde_json::json!({"status": "success", "message": "Room was created"});
            HttpResponse::Created()
                .append_header((
                    http::header::LOCATION,
                    HeaderValue::from_str(next_code.as_str()).unwrap(),
                ))
                .json(note_response)
        }
        Err(e) => HttpResponse::Ok()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[patch("/room/{code}")]
pub async fn patch_room(
    path: web::Path<String>,
    body: web::Json<UpdateRoom>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(RoomModel, "SELECT * FROM room WHERE code = $1", code)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Room with code: {} not found", code);
        return HttpResponse::Ok().json(serde_json::json!({"status": "fail","message": message}));
    }

    //let now = Utc::now();
    //let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        RoomModel,
        "UPDATE room SET name = $2, room_type = $3, description = $4 WHERE code = $1 RETURNING *",
        code,
        body.name.to_owned(),
        body.room_type.to_owned(),
        body.description.to_owned(),
    )
    .fetch_one(&data.db)
    .await;
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "room": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/room/{code}")]
pub async fn delete_room(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(
        RoomModel,
        "UPDATE room SET _removed = true WHERE code = $1 RETURNING *",
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
