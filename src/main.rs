mod server;
mod db;

/// all init actions
fn init() {
    env_logger::init();
}

#[actix_web::main]
async fn main() {
    init();

    server::start().await.expect("Web server start failed");
}