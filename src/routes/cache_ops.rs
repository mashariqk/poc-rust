use crate::AppStateWithCache;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Cache {
    key: String,
    value: String,
}

pub async fn get_all_data(cache: web::Data<AppStateWithCache>) -> impl Responder {
    let mut all_data = vec![];
    let cache = cache.get_cache().lock().unwrap();
    for (key, val) in cache.iter() {
        all_data.push(Cache {
            key: String::from(key),
            value: String::from(val),
        })
    }
    HttpResponse::Ok().json(all_data)
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
