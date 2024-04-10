use std::{fs::File, io::BufReader};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use log::info;

use crate::api;
use crate::config::Config;
use crate::db;

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

    // connect to database
    let web_db = db::connect(&config.db.provider,
                             &config.db.username,
                             &config.db.passwd,
                             &config.db.host,
                             &config.db.port,
                             &config.db.name).await.unwrap();

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

        // load TLS key/cert files
        let mut cert_file = BufReader::new(File::open(&config.server.cert).unwrap());
        let mut key_file = BufReader::new(File::open(&config.server.cert_key).unwrap());


        // load TLS certs and key
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
        let tls_certs = rustls_pemfile::certs(&mut cert_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let tls_key = rustls_pemfile::rsa_private_keys(&mut key_file)
            .next()
            .expect("get tls_key failed")
            .unwrap();

        // set up TLS config options
        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs1(tls_key))
            .unwrap();

        // listen to https in prod mode
        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin(config.server.hostname.as_str())
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(web_db.clone()))
                .configure(services)
        }).bind_rustls_0_22(("0.0.0.0", 8080), tls_config)?.run().await
    }
}