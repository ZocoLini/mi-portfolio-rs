use actix_web::{
    HttpRequest, HttpResponse,
    cookie::{Cookie, time},
    get,
    http::header,
    web,
};
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::{AppState, error::Error};

#[derive(Deserialize)]
struct PortfolioRedirectQuery {
    r: Option<String>,
}

#[allow(unused)]
#[derive(FromRow)]
struct QrScan {
    id: u64,
    reference: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    session_id: String,
    ip: Option<String>,
}

#[get("/api/v1/portfolio/qr")]
pub async fn portfolio_qr(
    req: HttpRequest,
    query: web::Query<PortfolioRedirectQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let mut response = HttpResponse::TemporaryRedirect();

    #[cfg(debug_assertions)]
    {
        response.insert_header((header::LOCATION, "https://127.0.0.1/portfolio"));
    }

    #[cfg(not(debug_assertions))]
    {
        response.insert_header((header::LOCATION, "https://bcastellano.com/portfolio"));
    }

    let session_cookie = req.cookie("session");

    let session_id = if let Some(cookie) = session_cookie {
        cookie.value().to_string()
    } else {
        let new_session = uuid::Uuid::new_v4().to_string();

        let cookie = Cookie::build("session", new_session.clone())
            .path("/")
            .max_age(time::Duration::MAX)
            .http_only(true)
            .finish();

        response.cookie(cookie);
        new_session
    };

    {
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .map(|ip| ip.to_string());
        let date_time = chrono::Utc::now();

        let query = sqlx::query!(
            "insert into qr_scan (reference, created_at, session_id, ip) values ($1, $2, $3, $4)",
            query.r,
            date_time,
            session_id,
            ip
        );

        query.execute(&state.db_pool).await?;
    }

    Ok(response.finish())
}
