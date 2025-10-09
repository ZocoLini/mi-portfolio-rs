use std::io;

use actix_web::{App, HttpResponse, HttpServer, get, http::header, middleware, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct PortfolioRedirectQuery {
    name: Option<String>,
}

#[get("/api/v1/portfolio/qr")]
async fn portfolio_qr(query: web::Query<PortfolioRedirectQuery>) -> HttpResponse {
    let query = &*query;
    let req_id = match query.name.as_ref() {
        Some(name) => name,
        None => "unknown",
    };

    if !req_id.contains("abc123.") {
        println!("z: {}", req_id);
    }

    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, "https://lebastudios.org/portfolio"))
        .finish()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Listening on 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(portfolio_qr)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
