use std::net::TcpListener;
use dotenv::dotenv;
use log::info;
use svr::{run, infrastructure::utils::logger};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 加载 .env 文件
    dotenv().ok();
    
    // 初始化日志系统
    logger::init_logger();
    
    // 从环境变量获取配置
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let db_name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());
    
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address)?;
    
    info!("Starting server on: {}", address);
    run(listener, &db_name).await?.await
}
