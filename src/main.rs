use actix_web::{web, App, HttpServer};
use rust_at_one::{
    handlers::configure_routes, mongo::Mongo, ssl_builder, AppConfig, AppEnv, AppState, Result,
};

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = AppConfig::new(AppEnv::Default);
    run_application(config).await
}

async fn run_application(config: AppConfig) -> Result<()> {
    let mongo = Mongo::initialize(config.mongo_db_uri.as_str(), config.db_name.as_str()).await?;
    let app_state = AppState::new(&mongo).wrap();
    let ssl = ssl_builder(config.cert_pem.as_str(), config.key_pem.as_str())?;

    HttpServer::new(move || {
        let m = App::new()
            .app_data(app_state.clone())
            .service(web::scope("/api").configure(configure_routes));
        m
    })
    .bind_openssl(config.ip_address, ssl)?
    .run()
    .await
    .map_err(|c| c.into())
}
