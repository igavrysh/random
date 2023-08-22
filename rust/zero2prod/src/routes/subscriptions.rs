use actix_web::{HttpResponse, web};
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
#[warn(dead_code)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    log::info!("request_id {} - Saving new subscrier details in the database", request_id);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await {
        Ok(_) => {
            log::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}