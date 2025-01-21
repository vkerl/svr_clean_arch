use std::net::TcpListener;

use actix_web::dev::Server;

mod api;
pub mod infrastructure;
mod app_state;
mod entities;
mod services;

pub async fn run(addr: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(addr, db_name).await
}