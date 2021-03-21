use crate::util::write_to_terminal_multicolor;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io;
use crate::configs::{AppConfig, GlobalServerState};
use crate::protocols::http::HttpProtocol;

pub struct LoadResponse {}

pub struct StressResponse {}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("Server Online")
}

#[post("/execute")]
async fn execute(context: web::Json<AppConfig<HttpProtocol>>) -> impl Responder {
    HttpResponse::Ok().json(context.into_inner())
}

pub async fn ignite_web_server() -> std::io::Result<()> {

    write_to_terminal_multicolor("Starting up server...");
    HttpServer::new(|| {
        App::new()
            .data(GlobalServerState {
                app_name: String::from("Actix-web"),
            })
            .service(execute)
            .service(status)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}