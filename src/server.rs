use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, post, Responder};

/// get user info
#[post("/api/getUserInfo")]
async fn user_info(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


/// start listening get and post request
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(user_info)
    }).bind(("127.0.0.1", 8080))?.run().await
}