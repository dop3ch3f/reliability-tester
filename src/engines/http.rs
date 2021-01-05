// Reliability Tester Http Engine
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_utils;
extern crate reqwest;
extern crate serde_json;

use crossbeam_utils::sync::WaitGroup;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::configs::{AppConfig, HttpMethods, RequestStatus};
use crate::protocols::http::HttpProtocol;
use crate::util::write_to_terminal_multicolor;
use std::collections::hash_map::RandomState;
use std::time::Instant;
use failure::Fail;

pub struct HttpEngine {}

impl HttpEngine {
    pub fn new(config: AppConfig<HttpProtocol>) {
        // process from config and run test
        if config.process_config.is_load_test.clone() {
            HttpEngine::load_test(config.clone())
        } else {
            HttpEngine::stress_test(config.clone())
        }
    }
    pub fn load_test(config: AppConfig<HttpProtocol>) {
        write_to_terminal_multicolor("load test initiated!").expect("unable to write to terminal");
        // spawn threads per hits at once
        let results_vec = spawn_threads(config.clone(), config.process_config.hits.clone() as u32);
        // parse results
        let results = prepare_result(results_vec);
        write_to_terminal_multicolor("load test complete!").expect("unable to write to terminal");
        // print results
        print_result(results);
    }
    pub fn stress_test(config: AppConfig<HttpProtocol>) {
        write_to_terminal_multicolor("stress test initiated!")
            .expect("unable to write to terminal");
        // spawn threads per hits at once with timer
        let results_vec =
            spawn_threads_with_timeout(config.clone(), config.process_config.hits.clone() as u32);
        // parse results
        let results = prepare_result(results_vec);
        write_to_terminal_multicolor("stress test complete!").expect("unable to write to terminal");
        // print results
        print_result(results);
    }
}

// fn to spawn threads of requests
pub fn spawn_threads(
    config: AppConfig<HttpProtocol>,
    number: u32,
) -> Vec<HashMap<&'static str, String, RandomState>> {
    // create an arc vector to work safely across threads
    let result = Arc::new(Mutex::new(Vec::new()));
    // create global wait group for operation
    let wg = WaitGroup::new();

    // loop to generate threads and attach to global wait group
    for _ in 0..number {
        let wg = wg.clone();
        let result_pointer = Arc::clone(&result);
        let config_clone = config.clone();

        thread::spawn(move || {
            let request_status = generate_request(config_clone.input_config.protocol.clone());
            let mut result_vector = result_pointer.lock().unwrap();
            result_vector.push(request_status);
            drop(wg)
        });
    }

    wg.wait();

    return result.lock().unwrap().to_vec();
}

// fn to spawn threads of requests within a time range and destroy all threads after time range expiry
pub fn spawn_threads_with_timeout(
    config: AppConfig<HttpProtocol>,
    number: u32,
) -> Vec<HashMap<&'static str, String, RandomState>> {
    // get end duration of stress test loop
    let mut end_duration = config.process_config.duration.clone();
    // start initial timer
    let start = Instant::now();
    // set end duration in reference to initial timer
    end_duration = end_duration + start.elapsed();
    // final results holder
    let result = Arc::new(Mutex::new(Vec::new()));
    loop {
        // global wait group
        let wg = WaitGroup::new();
        // check if time elapsed is still before target end
        if start.elapsed() < end_duration {
            // if still valid dispatch band of hits per loop
            for _ in 0..number {
                let wg_clone = wg.clone();
                let result_pointer = Arc::clone(&result);
                let config_clone = config.clone();

                thread::spawn(move || {
                    let request_status =
                        generate_request(config_clone.input_config.protocol.clone());
                    let mut result_vector = result_pointer.lock().unwrap();
                    result_vector.push(request_status);
                    drop(wg_clone)
                });
            }
        } else {
            // if time elapsed is greater than target end, break out of loop
            break;
        }
        wg.wait();
    }

    // return results
    return result.lock().unwrap().to_vec();
}

// fn to generate a simple blocking request
pub fn generate_request(config: HttpProtocol) -> HashMap<&'static str, String> {
    let mut response_map: HashMap<&str, String> = HashMap::new();
    let client_builder = reqwest::blocking::Client::builder()
        .timeout(config.timeout)
        .build();
    if client_builder.is_ok() {
        let client = client_builder.unwrap();
        // request generate match
        let resp: Result<reqwest::blocking::Response, reqwest::Error> = match config.method {
            HttpMethods::GET => client.get(config.url.as_str()).send(),
            _ => client.get(config.url.as_str()).send(),
        };
        if resp.is_ok() {
            let response = resp.unwrap();
            // if status code is successful
            if response.status().is_success() {
                // println!("success = {:?}", response);
                response_map.insert("status", "success".parse().unwrap());
                response_map.insert(
                    "data",
                    response
                        .text()
                        .expect("Unable to convert to string")
                        .clone(),
                );
            } else {
                // println!("error = {:?}", response);
                response_map.insert("status", "failure".parse().unwrap());
                response_map.insert(
                    "data",
                    response
                        .text()
                        .expect("Unable to convert to string")
                        .clone(),
                );
            }
        } else {
            response_map.insert("status", "failure".parse().unwrap());
            let response = resp.unwrap_err();
            // println!("status = {:?}", response);
            response_map.insert(
                "data",
                response.to_string(),
            );
        }
    }

    return response_map;
}

// fn to parse received vector of hashmap into final result hashmap
pub fn prepare_result<'a>(
    result_info: Vec<HashMap<&str, String, RandomState>>,
) -> HashMap<&'a str, f64> {
    let mut success_count: f64 = 0.0;
    let mut failed_count: f64 = 0.0;
    let mut percentage_successful: f64 = 0.0;

    for i in result_info.clone() {
        if i[&"status"] == String::from("success") {
            success_count += 1.0;
        }

        if i[&"status"] == String::from("failure") {
            failed_count += 1.0
        }

        percentage_successful = (success_count / (success_count + failed_count)) * 100.0
    }

    let mut results = HashMap::new();
    results.insert("Successful Requests", success_count);
    results.insert("Failed Requests", failed_count);
    results.insert("Percentage Successful", percentage_successful.round());
    results.insert("Total Requests", result_info.len() as f64);
    return results;
}

// fn to print final result hashmap
pub fn print_result(results: HashMap<&str, f64>) {
    write_to_terminal_multicolor("Results:").expect("unable to write to terminal");
    // print each entry of the result
    for (title, value) in &results {
        write_to_terminal_multicolor(format!("{t}: {v}", t = title, v = value).as_ref())
            .expect("unable to write to terminal");
    }
}
