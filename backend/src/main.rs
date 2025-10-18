use actix_web::{App, HttpServer, web};

use crate::error::Error;

mod endpoint;
mod db;
mod error;
mod middleware;

struct AppState {
    db_pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Setting up database");

    let sqlite_pool = db::connect().await?;
    db::prepare(&sqlite_pool).await?;

    let data = web::Data::new(AppState {
        db_pool: sqlite_pool,
    });

    println!("Listening on 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(middleware::SessionCookie)
            .service(
                web::scope("/api/v1/portfolio")
                    .service(web::scope("/qr").
                        service(endpoint::qr::qr))
                    .service(web::scope("/track")
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
