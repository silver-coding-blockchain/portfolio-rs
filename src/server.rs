use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::db;
use crate::api;

/// start listening get and post request
pub async fn start() -> std::io::Result<()> {
    let mode = "dev";

    if mode == "dev" {
        info!("🪜 Starting in 👷 dev mode 🛠️");

        let host = std::env::var("db_host").expect("Add your database address in env first");
        let port = std::env::var("db_port").expect("Add your database port in env first");
        let db = std::env::var("db_name").expect("Add your database port in env first");
        let username = std::env::var("db_username").expect("Add your username in env first");
        let password = std::env::var("db_password").expect("Add your password in env first");

        // connect to database
        let web_db = db::connect("postgres", &username, &password, &host, &port, &db).await.unwrap();


        // listen to http in dev mode
        HttpServer::new(move || {
            // no access control when in dev mode
            let cors = Cors::permissive();

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(web_db.clone()))
                .service(api::platform_links)
                .service(api::artist_info)
                .service(api::track_info)
        }).bind(("127.0.0.1", 8080))?.run().await
    } else {
        info!("💵 Starting in 😱 !prod! mode 🚨");

        let host = String::from("localhost");
        let port = String::from("5432");

        println!("Enter db_name:");
        let mut db = String::new();
        let _ = std::io::stdin().read_line(&mut db).unwrap();

        println!("Enter username:");
        let mut username = String::new();
        let _ = std::io::stdin().read_line(&mut username).unwrap();

        println!("Enter password:");
        let mut password = String::new();
        let _ = std::io::stdin().read_line(&mut password).unwrap();

        // connect to database
        let web_db = db::connect("postgres", &username, &password, &host, &port, &db).await.unwrap();


        // my ssl cert
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("portfolio.cloudiful.cn.key", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("portfolio.cloudiful.cn_bundle.pem").unwrap();

        // listen to https in prod mode
        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin("https://portfolio.cloudiful.cn")
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(web_db.clone()))
                .service(api::platform_links)
                .service(api::artist_info)
                .service(api::track_info)
        }).bind_openssl(("0.0.0.0", 8080), builder)?.run().await
    }
}