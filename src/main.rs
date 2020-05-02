// Reliability Tester
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_utils;
extern crate reqwest;

use reqwest::{
    blocking::{get, Response},
    StatusCode,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// struct for application configuration
pub struct AppConfig<I> {
    input_config: InputConfig<I>,
    process_config: ProcessConfig,
    output_config: OutputConfig,
}

// struct for data received from input channels, namely : console, http e.t.c
pub struct InputConfig<I> {
    protocol: I,
}

// struct for processing configurations and settings namely:  hits e.t.c
pub struct ProcessConfig {
    hits: u8,
}

// struct for output configurations and settings namely: file,console,loggine enabled e.t.c
pub struct OutputConfig {
    console: bool,
    logging: bool,
}

// struct for when protocol is Http
pub struct HttpProtocol {
    method: String,
    url: String,
    headers: HashMap<String, String>,
}

impl HttpProtocol {
    pub fn new(method: &str, url: &str, headers: HashMap<String, String>) -> HttpProtocol {
        HttpProtocol {
            method: String::from(method),
            url: String::from(url),
            headers: headers,
        }
    }
}

// engine for running tests on http protocol
fn http_engine(
    hits: u8,
    method: &str,
    url: &str,
) -> Result<HashMap<&'static str, f32>, Box<dyn std::error::Error>> {
    let success_count = Arc::new(Mutex::new(0.0));
    let failed_count = Arc::new(Mutex::new(0.0));
    let percentage_uptime = Arc::new(Mutex::new(0.0));
    let loop_value = hits;

    crossbeam::scope(|s| {
        for _ in 0..loop_value {
            let success_pointer = Arc::clone(&success_count);
            let failed_pointer = Arc::clone(&failed_count);
            let percentage_pointer = Arc::clone(&percentage_uptime);

            s.spawn(move |_| {
                // request generate match
                let resp: reqwest::Result<Response> = match method {
                    "get" => get(url),
                    _ => get(url),
                };
                let mut success = success_pointer.lock().unwrap();
                let mut failed = failed_pointer.lock().unwrap();
                let mut percentage = percentage_pointer.lock().unwrap();
                // if no errors
                if resp.is_ok() {
                    let resp_value = resp.unwrap();
                    // if status code is successful
                    if resp_value.status() == StatusCode::OK {
                        *success += 1.0;
                    } else {
                        *failed += 1.0;
                    }
                } else {
                    *failed += 1.0;
                }
                *percentage = (*success / (*success + *failed)) * 100.0;
            });
        }
    })
    .unwrap();
    // prepare results
    let mut results = HashMap::new();
    results.insert("Successful Requests: ", *success_count.lock().unwrap());
    results.insert("Failed Requests: ", *failed_count.lock().unwrap());
    results.insert("Percentage Uptime: ", *percentage_uptime.lock().unwrap());
    results.insert("Total Requests: ", hits as f32);
    Ok(results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate test http config
    let http_config: HttpProtocol =
        HttpProtocol::new("get", "https://httpbin.org/ip", HashMap::new());
    // generate input config
    let input_config: InputConfig<HttpProtocol> = InputConfig {
        protocol: http_config,
    };
    // generate process config
    let process_config: ProcessConfig = ProcessConfig { hits: 8 };
    // generate output config
    let output_config: OutputConfig = OutputConfig {
        logging: true,
        console: true,
    };
    // generate operation config
    let config: AppConfig<HttpProtocol> = AppConfig {
        input_config: input_config,
        process_config: process_config,
        output_config: output_config,
    };

    let results = http_engine(
        config.process_config.hits,
        config.input_config.protocol.method.as_str(),
        config.input_config.protocol.url.as_str(),
    )?;
    println!("final output: {:?}", results);
    Ok(())
}
