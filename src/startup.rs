use crate::routes::{add_data, clear_all_data, get_all_data, health_check, home};
use crate::AppStateWithCache;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use lru::LruCache;
use std::net::TcpListener;
use std::sync::Mutex;

pub fn run(
    listener: TcpListener,
    cache: LruCache<String, String>,
) -> Result<Server, std::io::Error> {
    let cache = web::Data::new(AppStateWithCache {
        cache: Mutex::new(cache),
    });
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(home))
            .route("/health", web::get().to(health_check))
            .route("/all-data", web::get().to(get_all_data))
            .route("/add", web::post().to(add_data))
            .route("/clear-cache", web::delete().to(clear_all_data))
            .app_data(cache.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
