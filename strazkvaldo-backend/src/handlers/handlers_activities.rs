use crate::application_enums::Periodicity;
use crate::model::{
    FinishedActivityResponse, OneTimeActivityModel, RepeatedActivityModel, UpcomingActivity,
    UpcomingActivityResponse,
};
use crate::schema::{FilterOptions, FinishedActivityModel, ReviewFinishedActivityModel};
use crate::AppState;
use actix_web::http::header::*;
use actix_web::{get, patch, web, HttpResponse, Responder};
use chrono::{Datelike, Local, Months, NaiveDate, TimeDelta, Utc};
use cron::Schedule;
use sqlx::PgPool;
use std::ops::Add;
use std::str::FromStr;
use std::sync::Arc;

fn filter_db_record(finished_activity_model: &FinishedActivityModel) -> FinishedActivityResponse {
    FinishedActivityResponse {
        code: finished_activity_model.code.clone(),
        repeated_activity_code: finished_activity_model.repeated_activity_code.clone(),
        one_time_activity_code: finished_activity_model.one_time_activity_code.clone(),
        due_date: finished_activity_model.due_date.clone(),
        description: finished_activity_model.description.clone(),
    }
}
fn from_cron(cron: String) -> NaiveDate {
    Schedule::from_str(cron.as_str())
        .unwrap()
        .upcoming(Utc)
        .next()
        .unwrap()
        .date_naive()
}

fn get_next_occurence_with_offset(
    periodicity: &Periodicity,
    date: &NaiveDate,
    offset: i32,
) -> NaiveDate {
    match periodicity {
        Periodicity::Day => date.add(TimeDelta::days(offset as i64)),
        Periodicity::Week => date.add(TimeDelta::weeks(offset as i64)),
        Periodicity::Month => {
            if offset >= 0 {
                date.checked_add_months(Months::new(offset as u32)).unwrap()
            } else {
                date.checked_sub_months(Months::new((-offset) as u32))
                    .unwrap()
            }
        }
        Periodicity::Year => {
            if offset >= 0 {
                date.checked_add_months(Months::new(12 * offset as u32))
                    .unwrap()
            } else {
                date.checked_sub_months(Months::new(12 * (-offset) as u32))
                    .unwrap()
            }
        }
    }
}

fn generate_occurences(
    periodicity: Periodicity,
    periodicity_unit: u32,
    future_count: u32,
) -> Vec<NaiveDate> {
    let next_occurence = match periodicity {
        Periodicity::Day => from_cron("0 0 0 * * * *".to_string()), // periodicity_unit is irrelevant
        Periodicity::Week => from_cron(format!("0 0 0 * * {periodicity_unit} *")), // periodicity_unit is day_of_week 0 = sunday, 6=saturday
        Periodicity::Month => from_cron(format!("0 0 0 {periodicity_unit} * * *")), // periodicity_unit is day_of_month 0 = sunday, 6=saturday
        Periodicity::Year => NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1)
            .unwrap()
            .add(TimeDelta::days(periodicity_unit as i64)), // periodicity_unit is day_of_year
    };

    let mut occurences = Vec::<NaiveDate>::with_capacity(future_count as usize);
    for x in -1..=(future_count as i32 - 1) {
        occurences.push(get_next_occurence_with_offset(
            &periodicity,
            &next_occurence,
            x,
        ));
    }

    // previous one can be out of range
    if occurences[0] < Local::now().naive_local().date() {
        occurences[1..].to_owned()
    } else {
        occurences[0..(occurences.len() - 1)].to_owned()
    }
}
#[get("/upcoming-activity")]
pub async fn get_upcoming_activities(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    //let limit = opts.limit.unwrap_or(10);
    //let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<OneTimeActivityModel> = sqlx::query_as!(
        OneTimeActivityModel,
        r#"SELECT * FROM one_time_activity where due_date <= CURRENT_DATE + interval '1 week' and due_date >= CURRENT_DATE"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let mut upcoming_activities = one_time_activities
        .into_iter()
        .map(|ota| UpcomingActivity {
            name: ota.name,
            repeated_activity_code: None::<String>,
            one_time_activity_code: Option::from(ota.code.clone()),
            due_date: ota.due_date,
        })
        .collect::<Vec<UpcomingActivity>>();

    let repeated_activities: Vec<RepeatedActivityModel> =
        sqlx::query_as!(RepeatedActivityModel, r#"SELECT * FROM repeated_activity"#)
            .fetch_all(&data.db)
            .await
            .unwrap();
    repeated_activities.iter().for_each(|ra| {
        generate_occurences(
            Periodicity::from_str(&ra.periodicity).unwrap(),
            ra.periodicity_unit as u32,
            3,
        )
        .iter()
        .for_each(|ca| {
            upcoming_activities.push(UpcomingActivity {
                name: ra.name.clone(),
                repeated_activity_code: Option::from(ra.code.clone()),
                one_time_activity_code: None::<String>,
                due_date: ca.to_owned(),
            });
        })
    });

    upcoming_activities.sort_by(|a, b| a.due_date.cmp(&b.due_date));
    let res: Vec<UpcomingActivityResponse> = upcoming_activities
        .iter()
        .map(|a| UpcomingActivityResponse {
            name: a.name.clone(),
            repeated_activity_code: a.repeated_activity_code.clone(),
            one_time_activity_code: a.one_time_activity_code.clone(),
            due_date: a.due_date.format("%d.%m.%Y").to_string(),
        })
        .collect();

    let json_response = serde_json::json!({
        "status": "success",
        "upcoming_activities": res
    });
    HttpResponse::Ok().json(json_response)
}

pub async fn generate_finished_activities_for_today(db: &PgPool) {
    let repeated_activities: Vec<RepeatedActivityModel> =
        sqlx::query_as!(RepeatedActivityModel, r#"SELECT * FROM repeated_activity"#)
            .fetch_all(db)
            .await
            .unwrap();

    let mut uas_all: Vec<UpcomingActivity> = repeated_activities
        .iter()
        .filter_map(|ra| {
            let next_occurence = generate_occurences(
                Periodicity::from_str(&ra.periodicity).unwrap(),
                ra.periodicity_unit as u32,
                1,
            )[0];
            if next_occurence.eq(&Local::now().date_naive()) {
                Some(UpcomingActivity {
                    name: ra.name.clone(),
                    repeated_activity_code: Option::from(ra.code.clone()),
                    one_time_activity_code: None::<String>,
                    due_date: next_occurence,
                })
            } else {
                None
            }
        })
        .collect();

    let one_time_activities: Vec<OneTimeActivityModel> = sqlx::query_as!(
        OneTimeActivityModel,
        r#"SELECT * FROM one_time_activity where DATE(due_date) = CURRENT_DATE"#
    )
    .fetch_all(db)
    .await
    .unwrap();

    let mut uas2: Vec<UpcomingActivity> = one_time_activities
        .iter()
        .map(|ota| UpcomingActivity {
            name: ota.name.clone(),
            repeated_activity_code: None::<String>,
            one_time_activity_code: Option::from(ota.code.clone()),
            due_date: ota.due_date,
        })
        .collect();
    uas_all.append(&mut uas2);

    let top_code: String =
        sqlx::query_scalar("SELECT code FROM finished_activity ORDER BY code DESC LIMIT 1")
            .fetch_one(db)
            .await
            .unwrap_or("FA-000000".to_owned());

    let next_code_number = top_code
        .strip_prefix("FA-")
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap()
        + 1;

    let query = r"
            INSERT INTO finished_activity
             (
                code,
                one_time_activity_code,
                repeated_activity_code,
                due_date,
                description,
                reviewed
            ) SELECT * FROM UNNEST(
                $1::CHAR(9)[],
                $2::CHAR(8)[],
                $3::CHAR(7)[],
                $4::DATE[],
                $5::TEXT[],
                $6::BOOLEAN[]
            ) ON CONFLICT DO NOTHING";

    let res = sqlx::query(query)
        .bind(Vec::from_iter(
            (0..uas_all.len()).map(|i| format!("FA-{:06}", next_code_number + (i as i32))),
        ))
        .bind(
            uas_all
                .iter()
                .map(|ua| ua.one_time_activity_code.clone())
                .collect::<Vec<Option<String>>>(),
        )
        .bind(
            uas_all
                .iter()
                .map(|ua| ua.repeated_activity_code.clone())
                .collect::<Vec<Option<String>>>(),
        )
        .bind(
            uas_all
                .iter()
                .map(|ua| ua.due_date.clone())
                .collect::<Vec<NaiveDate>>(),
        )
        .bind(vec!["".to_owned(); uas_all.len()])
        .bind(vec![false; uas_all.len()])
        .fetch_all(db)
        .await;

    match res {
        Ok(_) => {}
        Err(err) => {
            println!("Generating finshed activities failed: {err}")
        }
    };
}

#[get("/recently-finished-activity")]
pub async fn get_recently_finished_activity_list(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<FinishedActivityModel> = sqlx::query_as!(
        FinishedActivityModel,
        r#"SELECT * FROM finished_activity where reviewed = false LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let one_time_activities_response = one_time_activities
        .into_iter()
        .map(|note| FinishedActivityResponse {
            code: note.code,
            repeated_activity_code: note.repeated_activity_code,
            one_time_activity_code: note.one_time_activity_code,
            due_date: note.due_date,
            description: note.description,
        })
        .collect::<Vec<FinishedActivityResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "recently_finished_activities":one_time_activities_response
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/recently-finished-activity/{code}")]
pub async fn get_recently_finished_activity(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let code = path.into_inner();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        FinishedActivityModel,
        r#"SELECT * FROM finished_activity where code = $1"#,
        code
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(recently_finished_activity) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({ "recently_finished_activity": filter_db_record(&recently_finished_activity), })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Recently finished activity with ID: {} not found", code);
            return HttpResponse::Ok()
                .json(serde_json::json!({"status": "failure","message": message}));
        }
    }
}

#[patch("/recently-finished-activity/{code}/review")]
pub async fn review_recently_finished_activity(
    path: web::Path<String>,
    body: web::Json<ReviewFinishedActivityModel>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let code = path.into_inner();
    let query_result = sqlx::query_as!(
        FinishedActivityModel,
        "SELECT * FROM finished_activity WHERE code = $1",
        code
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Finished activity with code: {} not found", code);
        return HttpResponse::Ok().json(serde_json::json!({"status": "fail","message": message}));
    }

    //let now = Utc::now();
    //let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        FinishedActivityModel,
        "UPDATE finished_activity SET description = $2, reviewed = true WHERE code = $1 RETURNING *",
        code,
        body.description.to_owned(),
    ).fetch_one(&data.db)
        .await
        ;
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success"});

            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::Ok().json(serde_json::json!({"status": "error","message": message}))
        }
    }
}
#[get("/reviewed-finished-activity")]
pub async fn get_reviewed_finished_activity_list(
    data: web::Data<Arc<AppState>>,
    opts: web::Query<FilterOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let one_time_activities: Vec<FinishedActivityModel> = sqlx::query_as!(
        FinishedActivityModel,
        r#"SELECT * FROM finished_activity where reviewed = true LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let one_time_activities_response = one_time_activities
        .into_iter()
        .map(|item| filter_db_record(&item))
        .collect::<Vec<FinishedActivityResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "reviewed_finished_activities":one_time_activities_response
    });
    HttpResponse::Ok().json(json_response)
}
