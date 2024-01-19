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
        author_id: i32,
    }

    let req: Req = serde_json::from_str(&req_body).unwrap();

    let sql_str = format!("select pl.platform_id, p.platform_name, p.platform_icon, pl.link_url from portfolio.platforms p left join portfolio.platform_links pl on p.platform_id = pl.platform_id where author_id = {}", req.author_id);

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get author info
#[post("/api/getAuthorInfo")]
async fn author_info(req_body: String, web_db: web::Data<PgPool>) -> impl Responder {
    #[derive(Deserialize, Debug)]
    struct Req {
        author_id: i32,
    }
    let req: Req = serde_json::from_str(&req_body).unwrap();

    // query from database
    let res = db::query(&web_db, &format!("select * from portfolio.authors where author_id = {}", req.author_id)).await.unwrap();

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
        sql_str = format!("select a.track_name,
       a.track_name_cn,
       a.release_date,
       a.description,
       a.description_cn,
       c.author_name,
       json_agg(json_build_object('link_name', d.platform_name, 'link_url', b.link_url)) as links
from portfolio.tracks a
         left join portfolio.track_links b on a.track_id = b.track_id
         left join portfolio.authors c on c.author_id = any (a.author_id::int4[])
         left join portfolio.platforms d on b.platform_id = d.platform_id
where a.track_id = {}
group by a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, c.author_name", req.track_id.unwrap())
    } else if req.latest.is_some() {
        if req.latest.unwrap() {
            sql_str = String::from("select a.track_name,
       a.track_name_cn,
       a.release_date,
       a.description,
       a.description_cn,
       c.author_name,
       json_agg(json_build_object('link_name', d.platform_name, 'link_url', b.link_url)) as links
from portfolio.tracks a
         left join portfolio.track_links b on a.track_id = b.track_id
         left join portfolio.authors c on c.author_id = any (a.author_id::int4[])
         left join portfolio.platforms d on b.platform_id = d.platform_id
where release_date = (select max(release_date) from portfolio.tracks)
group by a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, c.author_name")
        }
    }

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get all tracks
#[post("/api/getAllTracks")]
async fn all_tracks(web_db: web::Data<PgPool>) -> impl Responder {
    let mut sql_str = String::new();

    sql_str = String::from("select a.track_name,
       a.track_name_cn,
       a.release_date,
       a.description,
       a.description_cn,
       c.author_name,
       json_agg(json_build_object('link_name', d.platform_name, 'link_url', b.link_url)) as links
from web_db.portfolio.tracks a
         left join web_db.portfolio.track_links b on a.track_id = b.track_id
         left join web_db.portfolio.authors c on c.author_id = any (a.author_id::int4[])
         left join web_db.portfolio.platforms d on b.platform_id = d.platform_id
group by a.track_name, a.track_name_cn, a.release_date, a.description, a.description_cn, c.author_name
order by release_date desc");

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

/// get all games
#[post("/api/getAllGames")]
async fn all_games(web_db: web::Data<PgPool>) -> impl Responder {
    let mut sql_str = String::new();

    sql_str = String::from("select a.game_name,
       a.game_name_cn,
       a.release_date,
       a.description,
       a.description_cn,
       array_agg(c.author_name),
       json_agg(json_build_object('link_name', d.platform_name, 'link_url', b.link_url)) as links
from web_db.portfolio.games a
         left join web_db.portfolio.game_links b on a.game_id = b.game_id
         left join web_db.portfolio.authors c on c.author_id = any (a.author_id::int4[])
         left join web_db.portfolio.platforms d on b.platform_id = d.platform_id
group by a.game_name, a.game_name_cn, a.release_date, a.description, a.description_cn
order by release_date desc");

    // query from database
    let res = db::query(&web_db, &sql_str).await.unwrap();

    let res_body = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().body(res_body)
}

