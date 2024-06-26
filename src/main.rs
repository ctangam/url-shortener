use actix_web::{get, web, App, HttpServer, Responder};

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

#[get("/")]
async fn index(data: web::Data<redis::Client>, json: web::Json<UrlReq>) -> impl Responder {
    let mut conn = data.get_connection().unwrap();
    "Hello, world!"
}

#[actix_web::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let client = web::Data::new(client.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(index)

    });
}
