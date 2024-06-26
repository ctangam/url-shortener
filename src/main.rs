use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use redis::Commands;
use serde::Serialize;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct UrlReq {
    long_url: String,
}

#[derive(serde::Serialize)]
struct UrlResp {
    key: String,
    long_url: String,
    short_url: String,
}

#[post("/store")]
async fn index(data: web::Data<redis::Client>, json: web::Json<UrlReq>) -> impl Responder {
    let long_url = json.long_url.clone();
    let key = Uuid::new_v4();
    let short_url = format!("http://localhost/{}", key);
    println!("{} -> {}", long_url, short_url);
    let resp = UrlResp {
        key: key.to_string(),
        long_url,
        short_url,
    };

    let mut conn = data.get_connection().unwrap();
    let s = serde_json::to_string(&resp).unwrap();
    let _: () = conn.set(key.to_string(), s).unwrap();
    HttpResponse::Ok().json(resp)
}

#[actix_web::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let client = web::Data::new(client.clone());
    HttpServer::new(move || App::new().app_data(client.clone()).service(index))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await;
}
