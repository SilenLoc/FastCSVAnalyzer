use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::csv_anaylzer::test_csv_analyze;


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/analyze")]
pub async fn analyze_csv(req_body: String) -> impl Responder {
    

    test_csv_analyze();
    HttpResponse::Ok().body(req_body)
}