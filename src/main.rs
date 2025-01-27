use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(seek);
    };

    Ok(config.into())
}
