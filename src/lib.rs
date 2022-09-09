use lru::LruCache;
use std::sync::Mutex;

pub mod routes;
pub mod startup;

pub struct AppStateWithCache {
    cache: Mutex<LruCache<String, String>>,
}

impl AppStateWithCache {
    pub fn new() -> AppStateWithCache {
        let cache: LruCache<String, String> = LruCache::new(100);
        AppStateWithCache {
            cache: Mutex::new(cache),
        }
    }

    pub fn get_cache(&self) -> &Mutex<LruCache<String, String>> {
        &self.cache
    }
}
