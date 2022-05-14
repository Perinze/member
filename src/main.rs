use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/json/{name}")]
async fn json(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(json))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}