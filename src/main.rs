use actix_web::{web, App, HttpServer};
use rust_at_one::{
    handlers::configure_routes, mongo::Mongo, ssl_builder, AppConfig, AppEnv, AppState, Result,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[actix_rt::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = AppConfig::new(AppEnv::Default);
    run_application(config).await
}

async fn run_application(config: AppConfig) -> Result<()> {
    let mongo = Mongo::initialize(config.mongo_db_uri.as_str(), config.db_name.as_str()).await?;
    let app_state = AppState::new(&mongo).wrap();

    let mut server = HttpServer::new(move || {
        let m = App::new()
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
