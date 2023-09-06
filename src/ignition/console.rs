extern crate clap;

use crate::configs::{AppConfig, HttpMethods, InputConfig, OutputConfig};
use crate::engines::http::HttpEngine;
use crate::protocols::http::HttpProtocol;
use crate::util::write_to_terminal_multicolor;
use std::io;
use std::time::Duration;
use serde_json::Map;

fn setup_http() -> HttpProtocol {
    let mut method = String::new();
    let mut url = String::new();
    let mut hits = String::new();
    let mut duration = String::new();
    write_to_terminal_multicolor("Input the method ?  [get/post/put/delete]").expect("TODO: panic message");
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
    write_to_terminal_multicolor("Input the url ? [any@any.any]").expect("TODO: panic message");
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    write_to_terminal_multicolor("Input the number of hits?  [1-9999]").expect("TODO: panic message");

    io::stdin()
        .read_line(&mut hits)
        .expect("Failed to read line");

    write_to_terminal_multicolor("Input the duration for stress test in seconds and zero for a load test?  [0-9999]").expect("TODO: panic message");

    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line");

    let http_config: HttpProtocol = HttpProtocol::new(
        match method.as_str() {
            "get" => HttpMethods::GET,
            "post" => HttpMethods::POST,
            "put" => HttpMethods::PUT,
            "delete" => HttpMethods::DELETE,
            _ => HttpMethods::GET,
        },
        url.as_str(),
        Map::new(),
        Map::new(),
        Duration::from_secs(120),
        hits.lines().next().unwrap().parse::<i32>().unwrap(),
        Duration::from_secs(duration.lines().next().unwrap().parse::<u64>().unwrap())
    );
    http_config
}

pub fn ignite_console() -> Result<(), Box<dyn std::error::Error>> {
    write_to_terminal_multicolor("Preparing Console Mode...").expect("TODO: panic message");
    write_to_terminal_multicolor(
        "Do you want to perform a stress test or load test?  [stress/load]",
    ).expect("TODO: panic message");

    let http_config = setup_http();

    // trigger the http engine
    HttpEngine::new(AppConfig {
        input_config: InputConfig {
            protocol: http_config,
        },
        output_config: OutputConfig {
            logging: true,
            console: true,
            file: true,
        },
    });

    Ok(())
}
