use std::future::{Ready, ready};

use actix_web::{
    Error, HttpMessage,
    cookie::{Cookie, time},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web::Data,
};
use futures_util::future::LocalBoxFuture;

use crate::AppState;

pub struct SessionCookie;

impl<S, B> Transform<S, ServiceRequest> for SessionCookie
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SessionCookieMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionCookieMiddleware { service }))
    }
}

pub struct SessionCookieMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SessionCookieMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (cookie_val, cookie) = if let Some(cookie) = req.cookie("session") {
            (cookie.value().to_string(), None)
        } else {
            let new_session = uuid::Uuid::new_v4().to_string();
            let cookie = Cookie::build("session", new_session.clone())
                .path("/")
                .max_age(time::Duration::days(365))
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Lax)
                .finish();

            (new_session, Some(cookie))
        };

        let data = req
            .app_data::<Data<AppState>>()
            .expect("server doesnt have app state")
            .clone();

        let session = bridge::Session::builder()
            .session_id(cookie_val)
            .last_activity(chrono::Utc::now())
            .build();

        req.extensions_mut().insert(session.clone());
        
        let fut = self.service.call(req);

        Box::pin(async move {
            insert_session(session, &data.db_pool).await.unwrap();

            let mut res = fut.await?;

            if let Some(cookie) = cookie {
                let _ = res.response_mut().add_cookie(&cookie);
            }

            Ok(res)
        })
    }
}

async fn insert_session(
    session: bridge::Session,
    db_pool: &sqlx::SqlitePool,
) -> Result<(), sqlx::Error> {
    let session_id = session.session_id();
    let last_activity = session.last_activity();

    let result = sqlx::query(r#"select session_id from session where session_id = $1"#)
        .bind(session_id)
        .fetch_optional(db_pool)
        .await?;

    if result.is_none() {
        sqlx::query!(
            r#"
                insert into session (session_id, last_activity)
                values ($1, $2)
            "#,
            session_id,
            last_activity
        )
        .execute(db_pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
                update session set last_activity = $1 where session_id = $2
            "#,
            last_activity,
            session_id
        )
        .execute(db_pool)
        .await?;
    }

    Ok(())
}
