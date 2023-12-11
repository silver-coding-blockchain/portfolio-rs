use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::db;

/// get user info
#[post("/api/getPlatformLinks")]
async fn get_platform_links(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    // request body struct
    #[derive(Serialize)]
    struct MusicPlatformLinks {
        netease: String,
        tencent: String,
        apple: String,
        spotify: String,
    }

    impl MusicPlatformLinks {
        fn new() -> MusicPlatformLinks {
            MusicPlatformLinks {
                netease: String::new(),
                tencent: String::new(),
                apple: String::new(),
                spotify: String::new(),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    struct Req {
        artist_name: String,
    }

    let req: Req = serde_json::from_str(&req_body).unwrap();

    let mut links = MusicPlatformLinks::new();

    // query from database
    let res = db::query(web_db.as_ref(),
                        &format!("select * from portfolio.platform_links left join portfolio.artists on portfolio.artists.artist_id = portfolio.platform_links.artist_id where portfolio.artists.artist_name = '{}'",req.artist_name)).await.unwrap();

    // print result
    for re in res {
        let platform_id = re.get("platform_id").unwrap().clone().to_i32().unwrap();

        match platform_id {
            1 => {
                links.netease = re.get("link_url").unwrap().clone().to_string().unwrap();
            }
            2 => {
                links.tencent = re.get("link_url").unwrap().clone().to_string().unwrap();
            }
            3 => {
                links.apple = re.get("link_url").unwrap().clone().to_string().unwrap();
            }
            4 => {
                links.spotify = re.get("link_url").unwrap().clone().to_string().unwrap();
            }
            _ => {}
        }

        info!("Query result: {:?}",re)
    }

    let res_body = serde_json::to_string(&links).unwrap();

    HttpResponse::Ok().body(res_body)
}


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
            .service(get_platform_links)
    }).bind(("127.0.0.1", 8080))?.run().await
}