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
#[derive(Debug, PartialEq)]
pub struct AppConfig {
    /// Mongo db connection string
    pub mongo_db_uri: String,
    /// Database to use
    pub db_name: String,
    /// SSL configs
    pub ssl_conf: Option<SSLConf>,
}

#[derive(Debug, PartialEq)]
pub struct SSLConf {
    pub lets_encrypt: Option<LetsEncryptConf>,
    pub key_pem: String,
    pub cert_pem: String,
}

#[derive(Debug, PartialEq)]
pub struct LetsEncryptConf {
    pub email: String,
    pub domain: String,
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

        Self::create(
            dotenv::var("MONGODB_URI"),
            dotenv::var("DB_NAME"),
            dotenv::var("USE_SSL"),
            dotenv::var("USE_LE"),
            dotenv::var("LE_EMAIL"),
            dotenv::var("LE_DOMAIN"),
            dotenv::var("KEY_PEM"),
            dotenv::var("CERT_PEM"),
        )
    }

    fn create(
        mongo_db: Result<String, dotenv::Error>,
        db_name: Result<String, dotenv::Error>,
        use_ssl: Result<String, dotenv::Error>,
        use_le: Result<String, dotenv::Error>,
        le_email: Result<String, dotenv::Error>,
        le_doman: Result<String, dotenv::Error>,
        key_pem: Result<String, dotenv::Error>,
        cert_pem: Result<String, dotenv::Error>,
    ) -> AppConfig {
        let mut config = AppConfig {
            mongo_db_uri: mongo_db.expect("MONGODB_URI was not found in environmental variables"),
            db_name: db_name.expect("DB_NAME was not found in environmental variables"),
            ssl_conf: None,
        };

        if use_ssl
            .map(|v| v.to_lowercase() == "true" || v.to_lowercase() == "1")
            .unwrap_or(false)
        {
            let mut ssl_conf = SSLConf {
                lets_encrypt: None,
                key_pem: key_pem
                    .expect("USE_SSL is true, KEY_PEM was not found in environmental variables"),
                cert_pem: cert_pem
                    .expect("USE_SSL is true, KEY_PEM was not found in environmental variables"),
            };

            if use_le
                .map(|v| v.to_lowercase() == "true" || v.to_lowercase() == "1")
                .unwrap_or(false)
            {
                ssl_conf.lets_encrypt = Some(LetsEncryptConf {
                    email: le_email.expect(
                        "USE_LE is true, LE_EMAIL was not found in environmental variables",
                    ),
                    domain: le_doman.expect(
                        "USE_LE is true, LE_DOMAIN was not found in environmental variables",
                    ),
                });
            }
            config.ssl_conf = Some(ssl_conf);
        }
        config
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
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::{self, Error};
    use std::{fs::File, io::Write};

    #[test]
    fn app_conf_env() {
        let mut file = File::create("app_conf_env.env").unwrap();
        file.write_all(
            b"
MONGODB_URI=a
DB_NAME=b
USE_SSL=true
USE_LE=true
LE_EMAIL=e
LE_DOMAIN=d
KEY_PEM=k
CERT_PEM=c",
        )
        .unwrap();
        dotenv::from_filename("app_conf_env.env").ok();

        let exp = AppConfig {
            mongo_db_uri: "a".to_string(),
            db_name: "b".to_string(),
            ssl_conf: Some(SSLConf {
                lets_encrypt: Some(LetsEncryptConf {
                    email: "e".to_string(),
                    domain: "d".to_string(),
                }),
                key_pem: "k".to_string(),
                cert_pem: "c".to_string(),
            }),
        };

        let conf = AppConfig::new(AppEnv::FromFile("app_conf_env.env"));
        std::fs::remove_file("app_conf_env.env").unwrap();
        assert_eq!(conf, exp);
    }

    #[test]
    fn app_conf_ssl_no_le() {
        let exp = AppConfig {
            mongo_db_uri: "a".to_string(),
            db_name: "b".to_string(),
            ssl_conf: Some(SSLConf {
                lets_encrypt: None,
                key_pem: "k".to_string(),
                cert_pem: "c".to_string(),
            }),
        };

        let conf = AppConfig::create(
            Ok("a".to_string()),
            Ok("b".to_string()),
            Ok("1".to_string()),
            Err(Error::LineParse("false".to_string(), 1)),
            Ok("".to_string()),
            Err(Error::LineParse("".to_string(), 1)),
            Ok("k".to_string()),
            Ok("c".to_string()),
        );

        assert_eq!(conf, exp);
    }

    #[test]
    fn app_conf_no_ssl() {
        let exp = AppConfig {
            mongo_db_uri: "a".to_string(),
            db_name: "b".to_string(),
            ssl_conf: None,
        };

        let conf = AppConfig::create(
            Ok("a".to_string()),
            Ok("b".to_string()),
            Err(Error::LineParse("0".to_string(), 1)),
            Ok("true".to_string()),
            Err(Error::LineParse("".to_string(), 1)),
            Ok("".to_string()),
            Err(Error::LineParse("".to_string(), 1)),
            Ok("c".to_string()),
        );

        assert_eq!(conf, exp);
    }

    #[test]
    #[should_panic(
        expected = "USE_SSL is true, KEY_PEM was not found in environmental variables: LineParse(\"\", 1)"
    )]
    fn app_conf_ssl_missing_conf() {
        AppConfig::create(
            Ok("a".to_string()),
            Ok("b".to_string()),
            Ok("1".to_string()),
            Err(Error::LineParse("".to_string(), 1)),
            Err(Error::LineParse("".to_string(), 1)),
            Ok("".to_string()),
            Err(Error::LineParse("".to_string(), 1)),
            Ok("c".to_string()),
        );
    }
}
