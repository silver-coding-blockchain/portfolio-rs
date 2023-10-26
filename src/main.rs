use log::info;
mod server;
mod db;

/// all init actions
fn init() {
    env_logger::init();
}

#[actix_web::main]
async fn main() {
    init();

    let host = std::env::var("db_host").expect("Add your database address in env first");
    let port = std::env::var("db_port").expect("Add your database port in env first");
    let db = std::env::var("db_name").expect("Add your database port in env first");
    let username = std::env::var("db_username").expect("Add your username in env first");
    let password = std::env::var("db_password").expect("Add your password in env first");

    // connect to database
    let web_db = db::connect("postgres", &username, &password, &host, &port, &db).await.unwrap();

    // query from database
    let res = db::query(web_db, "select * from portfolio.songs").await.unwrap();

    // print result
    for re in res {
        info!("Query result: {:?}",re)
    }

    // server::start().await.expect("Web server start failed");
}