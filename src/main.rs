use std::hash::{self, DefaultHasher, Hash, Hasher};

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use redis::Commands;

#[derive(serde::Deserialize)]
struct UrlReq {
    url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UrlResp {
    key: String,
    long_url: String,
    short_url: String,
}

#[post("/store")]
async fn store(data: web::Data<redis::Client>, json: web::Json<UrlReq>) -> impl Responder {
    let long_url = json.url.clone();
    let mut s = DefaultHasher::new();
    long_url.hash(&mut s);
    let key = s.finish();
    let mut conn = data.get_connection().unwrap();
    let stored: Option<String> = conn.get(key).unwrap();
    if let Some(s) = stored {
        println!("{s}");
        let s: UrlResp = serde_json::from_str(&s).unwrap();
        return HttpResponse::Ok().json(s)
    }
    let short_url = format!("http://localhost:8080/{}", key);
    println!("{} -> {}", long_url, short_url);
    let resp = UrlResp {
        key: key.to_string(),
        long_url,
        short_url,
    };

    let s: String = serde_json::to_string(&resp).unwrap();
    let _: () = conn.set(key.to_string(), s).unwrap();
    HttpResponse::Ok().json(resp)
}

#[get("/{key}")]
async fn fetch(data: web::Data<redis::Client>, path: web::Path<String>) -> impl Responder{
    let key = path.into_inner();
    let mut conn = data.get_connection().unwrap();
    let stored: Option<String> = conn.get(key).unwrap();
    if let Some(s) = stored {
        println!("{s}");
        let s: UrlResp = serde_json::from_str(&s).unwrap();
        return HttpResponse::Found().append_header(("location", s.long_url)).finish()
    };
    HttpResponse::NotFound().body(String::from("URL not found"))
}

#[delete("/{key}")]
async fn delete(data: web::Data<redis::Client>, path: web::Path<String>) -> impl Responder{
    let key = path.into_inner();
    let mut conn = data.get_connection().unwrap();
    let stored: Option<String> = conn.get_del(key).unwrap();
    if let Some(s) = stored {
        println!("{s}");
        return HttpResponse::Ok().finish()
    };
    HttpResponse::NotFound().body(String::from("URL not found"))

}


#[actix_web::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let client = web::Data::new(client.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(store)
            .service(fetch)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}