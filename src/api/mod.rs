use actix_web::web;

pub mod errors;
pub mod resp;
pub mod dto;
pub mod controllers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
                .route("/login", web::post().to(controllers::auth::login))
    );
}