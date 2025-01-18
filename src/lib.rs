use std::net::TcpListener;

use actix_web::dev::Server;

pub mod api;
pub mod infrastructure;
pub mod app_state;

pub async fn run(addr: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(addr, db_name).await
}