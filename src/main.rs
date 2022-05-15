use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

// cv handlers

async fn get_cv(
    cur_usr: 
) -> Result<impl Responder> {   // response is CvInfoResponse

    // code here
}

#[post("/updatestate")]
async fn update_state(
    cv_info:
    task:
    cur_usr:
) -> Result<impl Responder> {

    // code here
}

#[post("/updatesign")]
async fn update_sign(
    cv_info:
    task:
    cur_usr:
) -> Result<impl Responder> {

    // code here
}

#[post("/comment")]
async fn handle_comment(
    comment_dat:
    cur_usr:
) -> Result<impl Responder> {

    // code here
}

#[post("/getcomment")]
async fn get_comment(
    sno: &str,
    dept: &str,
    cur_usr: 
) -> Result<impl Responder> {

    // code here
}

fn cv_scope() -> web::Scope {
    web::scope("/cv")
        .service("/getinfo", web::get().to(get_cv))
        .service("/updatestate", web::post().to(update_state))
        .service("/updatesign", web::post().to(update_sign))
        .service("/comment", web::post().to(handle_comment))
        .service("/getcomment", web::post().to(get_comment))
}

// cv handlers end

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

    let app: App = App::new();

    HttpServer::new(|| App::new().service(json))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}