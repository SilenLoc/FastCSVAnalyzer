use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/analyze")]
pub async fn analyze_csv(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
