use actix_web::{HttpResponse, post, Responder, web};
use log::info;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db;

/// get music platform links
#[post("/api/getPlatformLinks")]
async fn platform_links(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    // request body struct
    #[derive(Deserialize, Debug)]
    struct Req {
        artist_id: i32,
    }

    info!("req_body: {}", req_body);

    let req: Req = serde_json::from_str(&req_body).unwrap();

    // query from database
    let res = db::query(&web_db, &format!("select pl.platform_id, p.platform_name, pl.link_url from portfolio.platforms p left join portfolio.platform_links pl on p.platform_id = pl.platform_id where artist_id = {}", req.artist_id)).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get artist info
#[post("/api/getArtistInfo")]
async fn artist_info(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    #[derive(Deserialize, Debug)]
    struct Req {
        artist_id: i32,
    }
    let req: Req = serde_json::from_str(&req_body).unwrap();

    // query from database
    let res = db::query(&web_db, &format!("select * from portfolio.artists where artist_id = {}", req.artist_id)).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get song info
#[post("/api/getTrackInfo")]
async fn track_info(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    #[derive(Deserialize, Debug)]
    struct Req {
        track_id: i32,
    }

    info!("req_body: {}", req_body);

    let req: Req = serde_json::from_str(&req_body).unwrap();

    // query from database
    let res = db::query(&web_db,
                        &format!(
                            "
                            select a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, b.link_url, c.artist_name
                            from portfolio.tracks a
                            left join portfolio.track_links b on a.track_id = b.track_id
                            left join portfolio.artists c on a.artist_id = c.artist_id
                            where a.track_id = {}", req.track_id)).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}
