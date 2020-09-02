use actix_web::web;

mod post_controller;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/{id}", web::get().to(post_controller::get_one))
            .route("/{id}", web::put().to(post_controller::put))
            .route("/{id}", web::delete().to(post_controller::delete))
            .route("", web::get().to(post_controller::get_many))
            .route("", web::post().to(post_controller::post)),
    );
}
