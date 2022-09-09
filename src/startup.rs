use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use lru::LruCache;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    cache: LruCache<String, String>,
) -> Result<Server, std::io::Error> {
    let cache = web::Data::new(cache);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .app_data(cache.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
