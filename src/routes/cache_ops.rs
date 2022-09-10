use crate::AppStateWithCache;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Cache {
    key: String,
    value: String,
}

pub async fn add_data(
    form: web::Json<Cache>,
    cache: web::Data<AppStateWithCache>,
) -> impl Responder {
    let mut cache = cache.get_cache().lock().unwrap();
    cache.put(form.key.clone(), form.value.clone());
    HttpResponse::Ok()
}

pub async fn clear_all_data(cache: web::Data<AppStateWithCache>) -> impl Responder {
    let mut cache = cache.get_cache().lock().unwrap();
    cache.clear();
    HttpResponse::Ok()
}

pub async fn get_value_for_key(
    path: web::Path<String>,
    cache: web::Data<AppStateWithCache>,
) -> impl Responder {
    let key = path.into_inner();
    let mut cache = cache.get_cache().lock().unwrap();
    match cache.get(&key) {
        Some(value) => HttpResponse::Ok().json(Cache {
            key,
            value: String::from(value),
        }),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn init_cache_with_size(
    path: web::Path<usize>,
    cache: web::Data<AppStateWithCache>,
) -> impl Responder {
    let mut cache = cache.get_cache().lock().unwrap();
    let size = path.into_inner();
    cache.clear();
    cache.resize(size);
    HttpResponse::Ok()
}

pub async fn cache_size(cache: web::Data<AppStateWithCache>) -> impl Responder {
    let cache = cache.get_cache().lock().unwrap();
    HttpResponse::Ok().json(cache.len())
}
