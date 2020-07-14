// Reliability Tester
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_utils;
extern crate reqwest;

use crossbeam_utils::sync::WaitGroup;
use reqwest::{
    blocking::{get, Response},
    StatusCode,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    hits: i32,
}

// struct for output configurations and settings namely: file,console,logging enabled e.t.c
pub struct OutputConfig {
    console: bool,
    logging: bool,
}

// struct for when protocol is Http
pub struct HttpProtocol {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    timeout: Duration,
}

impl HttpProtocol {
    pub fn new(
        method: &str,
        url: &str,
        headers: HashMap<String, String>,
        timeout: Duration,
    ) -> HttpProtocol {
        HttpProtocol {
            method: String::from(method),
            url: String::from(url),
            headers,
            timeout,
        }
    }
}

// engine for running tests on http protocol
fn http_engine(
    config_arc: &Arc<AppConfig<HttpProtocol>>,
) -> Result<HashMap<&'static str, i32>, Box<dyn std::error::Error>> {
    let success_count = Arc::new(Mutex::new(0));
    let failed_count = Arc::new(Mutex::new(0));
    let config_arc = config_arc.clone();
    let loop_value = config_arc.process_config.hits;

    let wg = WaitGroup::new();

    for i in 0..loop_value {
        // create another reference to the wait group
        let wg = wg.clone();
        let success_pointer = Arc::clone(&success_count);
        let failed_pointer = Arc::clone(&failed_count);
        let config = config_arc.clone();

        thread::spawn(move || {
            println!("spawning thread = {:?}", i);
            let new_config = &config.clone();
            let timeout = new_config.input_config.protocol.timeout;
            let method = &new_config.input_config.protocol.method;
            let url = &new_config.input_config.protocol.url;
            let client = reqwest::blocking::Client::builder()
                .timeout(timeout)
                .build()
                .unwrap();
            // request generate match
            let resp: Result<reqwest::blocking::Response, reqwest::Error> = match method.as_str() {
                "get" => client.get(url.as_str()).send(),
                _ => client.get(url.as_str()).send(),
            };
            let mut success = success_pointer.lock().unwrap();
            let mut failed = failed_pointer.lock().unwrap();
            if resp.is_ok() {
                let response = resp.unwrap();
                // if status code is successful
                if response.status().is_success() {
                    println!("success = {:?}", response);
                    *success += 1;
                } else {
                    println!("error = {:?}", response);
                    // println!("json = {:?}", resp.json<>)
                    *failed += 1;
                }
            } else {
                let response = resp.unwrap_err();
                println!("status = {:?}", response);
                // println!("json = {:?}", resp.json<>)
                *failed += 1;
            }
            drop(wg);
        });
    }

    // Block Until all threads have finished their work
    wg.wait();

    // crossbeam::scope(|s| {
    //     for i in 0..loop_value {
    //         let success_pointer = Arc::clone(&success_count);
    //         let failed_pointer = Arc::clone(&failed_count);
    //
    //         s.spawn(move |_| {
    //             println!("spawning thread = {:?}", i);
    //             let client = reqwest::blocking::Client::builder()
    //                 .timeout(timeout)
    //                 .build()
    //                 .unwrap();
    //             // request generate match
    //             let resp: Result<reqwest::blocking::Response, reqwest::Error> = match method {
    //                 "get" => client.get(url).send(),
    //                 _ => client.get(url).send(),
    //             };
    //             let mut success = success_pointer.lock().unwrap();
    //             let mut failed = failed_pointer.lock().unwrap();
    //             if resp.is_ok() {
    //                 let response = resp.unwrap();
    //                 // if status code is successful
    //                 if response.status().is_success() {
    //                     *success += 1.0;
    //                 } else {
    //                     println!("status = {:?}", response);
    //                     // println!("json = {:?}", resp.json<>)
    //                     *failed += 1.0;
    //                 }
    //             } else {
    //                 let response = resp.unwrap_err();
    //                 println!("status = {:?}", response);
    //                 // println!("json = {:?}", resp.json<>)
    //                 *failed += 1.0;
    //             }
    //         });
    //     }
    // })
    // .unwrap();

    let success = *success_count.lock().unwrap() as f32;
    let failed = *failed_count.lock().unwrap() as f32;
    let percentage_uptime: f32 = (success / (success + failed)) * 100.0;

    // prepare results
    let mut results = HashMap::new();
    results.insert("Successful Requests:", *success_count.lock().unwrap());
    results.insert("Failed Requests:", *failed_count.lock().unwrap());
    results.insert("Percentage Successful:", percentage_uptime as i32);
    results.insert("Total Requests:", config_arc.process_config.hits);
    return Ok(results);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate test http config
    let http_config: HttpProtocol = HttpProtocol::new(
        "get",
        "http://127.0.0.1:8000/api/public/lessons",
        // "https://www.google.com",
        HashMap::new(),
        Duration::from_secs(120),
    );
    // generate input config
    let input_config: InputConfig<HttpProtocol> = InputConfig {
        protocol: http_config,
    };
    // generate process config
    let process_config: ProcessConfig = ProcessConfig { hits: 5000 };
    // generate output config
    let output_config: OutputConfig = OutputConfig {
        logging: true,
        console: true,
    };
    // generate operation config
    let config: AppConfig<HttpProtocol> = AppConfig {
        input_config,
        process_config,
        output_config,
    };

    let config_arc = Arc::new(config);
    let results = http_engine(&config_arc).expect("error occurred here");
    println!("final output: {:?}", results);
    Ok(())
}
