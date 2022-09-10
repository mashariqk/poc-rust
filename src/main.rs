use env_logger::Env;
use lru::LruCache;
use poc_rust::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cache = LruCache::new(2);
    let address = "0.0.0.0:9001";
    let listener = TcpListener::bind(address).expect("Failed to bind");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run(listener, cache)?.await
}
