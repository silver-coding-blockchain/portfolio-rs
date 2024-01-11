use actix_web::{HttpResponse, post, Responder, web};
use serde::Deserialize;
use sqlx::PgPool;

use crate::db;

/// get music platform links
#[post("/api/getPlatformInfo")]
async fn platform_info(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    // request body struct
    #[derive(Deserialize, Debug)]
    struct Req {
        artist_id: i32,
    }

    let req: Req = serde_json::from_str(&req_body).unwrap();

    let sql_str = format!("select pl.platform_id, p.platform_name, p.platform_icon, pl.link_url from portfolio.platforms p left join portfolio.platform_links pl on p.platform_id = pl.platform_id where artist_id = {}", req.artist_id);

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

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
/// * 'track_id' - Search using track id
/// * 'latest' - Search latest track info while track_id is none
#[post("/api/getTrackInfo")]
async fn track_info(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    #[derive(Deserialize, Debug)]
    struct Req {
        track_id: Option<i32>,
        latest: Option<bool>,
    }

    let req: Req = serde_json::from_str(&req_body).unwrap();

    let mut sql_str = String::new();

    if req.track_id.is_some() {
        sql_str = format!("select a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, b.link_url, c.artist_name, d.platform_name
        from portfolio.tracks a
        left join portfolio.track_links b on a.track_id = b.track_id
        left join portfolio.artists c on a.artist_id = c.artist_id
        left join portfolio.platforms d on b.platform_id = d.platform_id
        where a.track_id = {}", req.track_id.unwrap())
    } else if req.latest.is_some() {
        if req.latest.unwrap() {
            sql_str = String::from("select a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, b.link_url, c.artist_name, d.platform_name
            from portfolio.tracks a
            left join portfolio.track_links b on a.track_id = b.track_id
            left join portfolio.artists c on a.artist_id = c.artist_id
            left join portfolio.platforms d on b.platform_id = d.platform_id
            where release_date = (select max(release_date) from portfolio.tracks)")
        }
    }

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get all tracks (with each link return a line)
#[post("/api/getAllTracks")]
async fn all_tracks(web_db: web::Data<PgPool>) -> impl Responder {
    let mut sql_str = String::new();

    sql_str = String::from("select a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, b.link_url, c.artist_name, d.platform_name
from web_db.portfolio.tracks a
left join web_db.portfolio.track_links b on a.track_id = b.track_id
left join web_db.portfolio.artists c on a.artist_id = c.artist_id
left join web_db.portfolio.platforms d on b.platform_id = d.platform_id
 order by release_date desc");

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

