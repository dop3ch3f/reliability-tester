// Reliability Tester Http Engine
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_utils;
extern crate reqwest;

use crossbeam_utils::sync::WaitGroup;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::configs::AppConfig;
use crate::protocols::http::HttpProtocol;

// engine for running tests on http protocol
pub fn http_engine(
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
