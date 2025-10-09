use actix_web::{middleware, web, App, HttpServer};

use crate::error::Error;

mod db;
mod api;
mod error;

struct AppState {
    db_pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    println!("Setting up database");
    
    let sqlite_pool = db::connect().await?;
    db::prepare(&sqlite_pool).await?;
    
    let data = web::Data::new(AppState {
        db_pool: sqlite_pool
    });
    
    println!("Listening on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Compress::default())
            .service(api::qr::portfolio_qr)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await?;
    
    Ok(())
}
