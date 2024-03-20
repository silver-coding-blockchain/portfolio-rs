use log::info;

mod server;
mod db;
mod api;
mod config;

/// all init actions
fn init() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("🚀 Initiating log4rs 📝");
}

#[actix_web::main]
async fn main() {
    init();

    server::start().await.expect("Web server start failed");
}