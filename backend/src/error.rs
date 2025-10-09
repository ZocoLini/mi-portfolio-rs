use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl ResponseError for Error {
    
}
