extern crate core;

use endpoints::analyze_csv;
use endpoints::hello;
use crate::init::init;
use crate::logging::init_logger;
use actix_web::{HttpServer, App};

mod analyze;
mod init;
mod logging;
mod outs;
mod predicates;
mod reader;
mod matcher;
mod endpoints;
mod csv_anaylzer;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
   
    HttpServer::new(|| {
        App::new()
            .service(analyze_csv)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


