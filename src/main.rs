use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use rust_at_one::{
    handlers::configure_routes, mongo::Mongo, ssl_builder, AppConfig, AppEnv, AppState, Result,
};

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = AppConfig::new(AppEnv::Default);
    env_logger::init();
    run_application(config).await
}

async fn run_application(config: AppConfig) -> Result<()> {
    let mongo = Mongo::initialize(config.mongo_db_uri.as_str(), config.db_name.as_str()).await?;
    let app_state = AppState::new(&mongo).wrap();

    let mut server = HttpServer::new(move || {
        let m = App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .app_data(app_state.clone())
            .service(web::scope("/api").configure(configure_routes));
        m
    });

    if let Some(c) = config.ssl_conf {
        let ssl = ssl_builder(c.cert_pem.as_str(), c.key_pem.as_str())?;
        if let Some(le) = c.lets_encrypt {
            // TODO: Let's encrypt
            dbg!(&le);
        }
        server = server.bind_openssl("0.0.0.0:8000", ssl)?;
    } else {
        server = server.bind("0.0.0.0:8000")?;
    };

    server.run().await.map_err(|c| c.into())
}
