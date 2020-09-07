#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Module for documents/models
pub mod documents;
/// Error types
pub mod error;
/// Endpoint handlers
pub mod handlers;
/// Mongo specific logic
pub mod mongo;
/// Abstraction layer, data manipulation logic
pub mod services;

use actix_web::web;
use dotenv::dotenv;
use error::AppError;
use mongo::Mongo;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use services::{post_service::PostService, DocumentService};

pub type Result<T, E = AppError> = core::result::Result<T, E>;

/// Configuration for the application
#[derive(Debug)]
pub struct AppConfig {
    /// Mongo db connection string
    pub mongo_db_uri: String,
    /// Database to use
    pub db_name: String,
    /// SSL certificate file
    pub cert_pem: String,
    /// SSL key file
    pub key_pem: String,
}

pub enum AppEnv<'a> {
    Default,
    FromFile(&'a str),
}

impl AppConfig {
    pub fn new(env: AppEnv) -> Self {
        match env {
            AppEnv::Default => {
                dotenv().ok();
            }
            AppEnv::FromFile(s) => {
                dotenv::from_filename(s).ok();
            }
        };

        AppConfig {
            mongo_db_uri: dotenv::var("MONGODB_URI")
                .expect("MONGODB_URI was not found in environmental variables"),
            db_name: dotenv::var("DB_NAME")
                .expect("DB_NAME was not found in environmental variables"),
            cert_pem: dotenv::var("CERT_PEM")
                .expect("CERT_PEM was not found in environmental variables"),
            key_pem: dotenv::var("KEY_PEM")
                .expect("KEY_PEM was not found in environmental variables"),
        }
    }
}

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
