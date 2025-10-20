use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, http::header, web};
use bridge::QrScan;
use serde::Deserialize;

use crate::{
    AppState,
    error::{self, Error},
};

#[derive(Deserialize)]
struct PortfolioRedirectQuery {
    r: Option<String>,
}

#[get("")]
pub async fn qr(
    req: HttpRequest,
    query: web::Query<PortfolioRedirectQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let mut response = HttpResponse::TemporaryRedirect();

    #[cfg(debug_assertions)]
    {
        response.insert_header((header::LOCATION, "http://127.0.0.1:8000/portfolio/"));
    }

    #[cfg(not(debug_assertions))]
    {
        response.insert_header((header::LOCATION, "https://bcastellano.com/portfolio/"));
    }

    let session = {
        let extensions = req.extensions();
        extensions.get::<bridge::Session>().cloned()
    };

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string());

    let date_time = chrono::Utc::now();

    let qr_scan = QrScan::builder()
        .reference(query.r)
        .created_at(date_time)
        .session_id(
            session
                .map(|c| c.session_id().to_string())
                .unwrap_or("internal-error".to_string()),
        )
        .ip(ip)
        .build();

    insert_qr_scan(qr_scan, &state.db_pool).await?;

    Ok(response.finish())
}

async fn insert_qr_scan(
    qr_scan: bridge::QrScan,
    db_pool: &sqlx::SqlitePool,
) -> Result<(), error::Error> {
    let reference = qr_scan.reference();
    let created_at = qr_scan.created_at();
    let session_id = qr_scan.session_id();
    let ip = qr_scan.ip();

    let query = sqlx::query!(
        "insert into qr_scan (reference, created_at, session_id, ip) values ($1, $2, $3, $4)",
        reference,
        created_at,
        session_id,
        ip
    );

    query.execute(db_pool).await?;

    Ok(())
}
