use crate::routes::{
    add_data, cache_size, clear_all_data, get_value_for_key, health_check, home,
    init_cache_with_size,
};
use crate::AppStateWithCache;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use lru::LruCache;
use std::net::TcpListener;
use std::sync::Mutex;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    cache: LruCache<String, String>,
) -> Result<Server, std::io::Error> {
    let cache = web::Data::new(AppStateWithCache {
        cache: Mutex::new(cache),
    });
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/health", web::get().to(health_check))
            .route("/add", web::post().to(add_data))
            .route("/get/{key}", web::get().to(get_value_for_key))
            .route("/clear-cache", web::delete().to(clear_all_data))
            .route("/init/{size}", web::post().to(init_cache_with_size))
            .route("/cache-size", web::get().to(cache_size))
            .app_data(cache.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
