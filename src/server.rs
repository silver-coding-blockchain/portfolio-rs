use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use crate::db;
use crate::info;

/// start listening get and post request
pub async fn start() -> std::io::Result<()> {
    let host = std::env::var("db_host").expect("Add your database address in env first");
    let port = std::env::var("db_port").expect("Add your database port in env first");
    let db = std::env::var("db_name").expect("Add your database port in env first");
    let username = std::env::var("db_username").expect("Add your username in env first");
    let password = std::env::var("db_password").expect("Add your password in env first");

    // connect to database
    let web_db = db::connect("postgres", &username, &password, &host, &port, &db).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(web_db.clone()))
            .service(info::platform_links)
            .service(info::artist_info)
            .service(info::track_info)
    }).bind(("127.0.0.1", 8080))?.run().await
}