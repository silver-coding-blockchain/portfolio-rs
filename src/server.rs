use std::{fs::File, io::BufReader};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use log::info;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};

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
        let cert_file = &mut BufReader::new(File::open(config.server.cert).unwrap());
        let key_file = &mut BufReader::new(File::open(config.server.cert_key).unwrap());

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

        // my ssl cert config
        let ssl_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, keys.remove(0)).unwrap();

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
        }).bind_rustls_021(("0.0.0.0", 8080), ssl_config)?.run().await
    }
}