pub mod documents;
pub mod error;
pub mod handlers;
pub mod mongo;
pub mod services;

use actix_web::web;
use error::AppError;
use mongo::Mongo;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use services::{post_service::PostService, DocumentService};

pub type Result<T, E = AppError> = core::result::Result<T, E>;

// load ssl keys
// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
// `openssl rsa -in key.pem -out nopass.pem`
pub fn ssl_builder(cert: &str, key: &str) -> Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert)?;
    Ok(builder)
}

pub struct AppState {
    pub post_service: PostService,
}

impl<'a> AppState {
    pub fn new(mongo: &Mongo) -> Self {
        let post_service = PostService::new(mongo);
        AppState { post_service }
    }

    /// Wrap the AppState in a actix_web::web::Data container
    pub fn wrap(self) -> web::Data<AppState> {
        web::Data::new(self)
    }
}
