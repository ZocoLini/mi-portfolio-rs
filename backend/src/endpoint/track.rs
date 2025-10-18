use actix_web::{post, web::{self, Json}, HttpMessage, HttpRequest, HttpResponse};
use bridge::ContentView;
use serde::Deserialize;

use crate::{error, AppState};

#[derive(Deserialize)]
struct ContentViewRequest {
    content_id: String,
}

#[post("/content-view")]
pub async fn content_view(
    req: HttpRequest,
    body: Json<ContentViewRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {

    let session = {
        let extensions = req.extensions();
        extensions.get::<bridge::Session>().cloned()
    };

    let data = ContentView::builder()
        .session_id(
            session
                .map(|c| c.session_id().to_string())
                .unwrap_or("internal-error".to_string()),
        )
        .content_id(body.content_id.clone())
        .build();

    insert_content_view(data, &state.db_pool).await?;

    Ok(HttpResponse::Ok().finish())
}

async fn insert_content_view(data: bridge::ContentView, db_pool: &sqlx::SqlitePool) -> Result<(), error::Error> {
    let reference = data.session_id();
    let created_at = data.content_id();
    
    let query = sqlx::query!(
        "insert into content_view (session_id, content_id) values ($1, $2)",
        reference,
        created_at
    );

    query.execute(db_pool).await?;

    Ok(())
}