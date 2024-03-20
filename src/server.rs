use std::{fs::File, io::BufReader};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use log::info;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};

use crate::api;
use crate::config::Config;
use crate::db;

fn rustls_config() -> ServerConfig {
    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("portfolio.cloudiful.cn_bundle.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("portfolio.cloudiful.cn.key").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

// store all the service in one place
fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(api::platform_info);
    cfg.service(api::author_info);
    cfg.service(api::track_info);
    cfg.service(api::all_tracks);
    cfg.service(api::all_games);
    cfg.service(api::all_videos);
}

/// start listening get and post request
pub async fn start() -> std::io::Result<()> {
    let config = Config::read();

    let host = config.db.host;
    let port = config.db.port;
    let db = config.db.name;
    let username = config.db.username;
    let password = config.db.passwd;

    // connect to database
    let web_db = db::connect("postgres", &username, &password, &host, &port, &db).await.unwrap();

    if config.mode == "dev" {
        info!("🪜 Starting in 👷 dev mode 🛠️");

        // listen to http in dev mode
        HttpServer::new(move || {
            // no access control when in dev mode
            let cors = Cors::permissive();

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(web_db.clone()))
                .configure(services)
        }).bind(("127.0.0.1", 8080))?.run().await
    } else {
        info!("💵 Starting in 😱 !prod! mode 🚨");

        // my ssl cert
        let config = rustls_config();

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
                .configure(services)
        }).bind_rustls_021(("0.0.0.0", 8080), config)?.run().await
    }
}