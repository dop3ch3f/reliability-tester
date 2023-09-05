use crate::util::write_to_terminal_multicolor;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io;
use crate::configs::{AppConfig, GlobalServerState, HttpMethods, InputConfig, OutputConfig};
use crate::protocols::http::{HttpProtocol, HttpProtocolMulti};
use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;
use crate::engines::http::HttpEngine;
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Clone )]
pub struct JobRequest {
    method: String,
    url: String,
    body_json: HashMap<String, Value>,
    headers: HashMap<String, String>,
    hits: i32,
    duration: i32,
    timeout: i32,
}

// enum ResponseType {
//     Json,
//     File,
// }

#[derive(Deserialize)]
pub struct ExecuteRequest {
    requests: Vec<JobRequest>,
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body(
        "\n
        Reliability Tester Server is Online\n
        Send your execution order to /execute with the following sample\n
        {
            requests: [{
                method: 'get/post/put/delete',
                url: 'http://localhost',
                body_json: your request body as JSON,
                headers: your request headers as JSON<String, String>,
                hits: 1-999,
                duration: duration in seconds (120)
            }]
        }
        "
    )
}

#[post("/execute")]
async fn execute(req: web::Json<ExecuteRequest>) -> impl Responder {

    let requests_config: Vec<HttpProtocol> = req.requests.clone().into_iter().map(|x| {
        HttpProtocol {
            method: match x.method.as_str() {
                "get" => HttpMethods::GET,
                "post" => HttpMethods::POST,
                "put" => HttpMethods::PUT,
                "delete" => HttpMethods::DELETE,
                _ => HttpMethods::GET,
            },
            url: x.url,
            headers: x.headers,
            body_json: x.body_json,
            timeout: Duration::from_secs(x.timeout as u64),
            hits: x.hits,
            duration: Duration::from_secs(x.duration as u64),
        }
    }).collect();

    let response = HttpEngine::new_multi(AppConfig {
        input_config: InputConfig {
            protocol: HttpProtocolMulti {
                requests: requests_config
            }
        },
        output_config: OutputConfig {
            logging: true,
            console: true,
            file: true,
        },
    });

    HttpResponse::Ok().json(Value::from(
        response.into_iter().map(
            |x: HashMap<String, Value>| {
                serde_json::to_string(&x).unwrap() as String
            }
        ).collect::<Vec<String>>()
    ))
}

pub async fn ignite_web_server() -> std::io::Result<()> {

    write_to_terminal_multicolor("Server Live @ Port 7373").expect("TODO: panic message");
    HttpServer::new(|| {
        App::new()
            .data(GlobalServerState {
                app_name: String::from("Actix-web"),
            })
            .service(execute)
            .service(status)
    })
        .bind("127.0.0.1:7373")?
        .run()
        .await
}