use lru::LruCache;
use poc_rust::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cache = LruCache::new(100);
    let address = "127.0.0.1:9001";
    let listener = TcpListener::bind(address).expect("Failed to bind");
    run(listener, cache)?.await
}
