#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[macro_use]
extern crate dotenv_codegen;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rust_at_one::{handlers::configure_post_routes, mongo::Mongo, ssl_builder, AppState, Result};

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    run_application().await
}

async fn run_application() -> Result<()> {
    let mongo = Mongo::initialize(dotenv!("MONGODB_URI")).await?;
    let app_state = AppState::new(&mongo).wrap();
    let ssl = ssl_builder(dotenv!("CERT_PEM"), dotenv!("KEY_PEM"))?;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/api").configure(configure_post_routes))
    })
    .bind_openssl(dotenv!("IP_ADDRESS"), ssl)?
    .run()
    .await
    .map_err(|c| c.into())
}
