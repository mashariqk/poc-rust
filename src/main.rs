use lru::LruCache;
use poc_rust::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cache = LruCache::new(2);
    let address = "0.0.0.0:9001";
    let listener = TcpListener::bind(address).expect("Failed to bind");
    run(listener, cache)?.await
}
