// Reliability Tester Http Engine
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_utils;
extern crate reqwest;
extern crate serde_json;

use crossbeam_utils::sync::WaitGroup;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::configs::{AppConfig, HttpMethods, InputConfig, OutputConfig};
use crate::protocols::http::{HttpProtocol, HttpProtocolMulti};
use crate::util::write_to_terminal_multicolor;

use std::time::Instant;


use reqwest::header::{HeaderValue, HeaderName};
use serde_json::{Map, Value};

pub struct HttpEngine {}

impl HttpEngine {
    pub fn new(config: AppConfig<HttpProtocol>) -> Map<String, Value> {
        // process from config and run test
        if config.input_config.protocol.duration.as_secs_f64() <= 0 as f64 {
            HttpEngine::load_test(config.clone())
        } else {
            HttpEngine::stress_test(config.clone())
        }
    }
    pub fn new_multi(config: AppConfig<HttpProtocolMulti>) -> Vec<Map<String, Value>> {
        write_to_terminal_multicolor("Multi test initiated!").expect("unable to write to terminal");
        // spawn threads for multiple instances of HttpEngine calls
        let result = Arc::new(Mutex::new(Vec::new()));
        let wg = WaitGroup::new();

        // loop to generate threads and attach to global wait group
        for i in 0..config.input_config.protocol.requests.len() as i32 {
            let wg = wg.clone();
            let result_pointer = Arc::clone(&result);
            let config_clone = config.clone();

            thread::spawn(move || {
                let request_config: HttpProtocol = (config_clone.input_config.protocol.requests.get(i as usize).unwrap()).clone();
                let http_config: HttpProtocol = HttpProtocol::new(
                    match request_config.method.to_string().as_str() {
                        "get" => HttpMethods::GET,
                        "post" => HttpMethods::POST,
                        "put" => HttpMethods::PUT,
                        "delete" => HttpMethods::DELETE,
                        _ => HttpMethods::GET,
                    },
                    request_config.url.as_str(),
                    request_config.headers,
                    request_config.body_json,
                    request_config.timeout,
                    request_config.hits,
                    request_config.duration
                );

                let output = HttpEngine::new(AppConfig {
                    input_config: InputConfig {
                        protocol: http_config,
                    },
                    output_config: OutputConfig {
                        logging: true,
                        console: true,
                        file: true,
                    }
                });

                let mut result_vector = result_pointer.lock().unwrap();
                result_vector.push(output);
                drop(wg)
            });
        }

        wg.wait();

        return result.lock().unwrap().to_vec();
    }
    pub fn load_test(config: AppConfig<HttpProtocol>) -> Map<String, Value> {
        write_to_terminal_multicolor("load test initiated!").expect("unable to write to terminal");
        // spawn threads per hits at once
        let results_vec = spawn_threads(config.clone(), config.input_config.protocol.hits.clone() as u32);
        // parse results
        let results = prepare_result(results_vec);
        write_to_terminal_multicolor("load test complete!").expect("unable to write to terminal");
        // print results
        print_result(results.clone());
        return results;
    }
    pub fn stress_test(config: AppConfig<HttpProtocol>) -> Map<String, Value> {
        write_to_terminal_multicolor("stress test initiated!")
            .expect("unable to write to terminal");
        // spawn threads per hits at once with timer
        let results_vec =
            spawn_threads_with_timeout(config.clone(), config.input_config.protocol.hits.clone() as u32);
        // parse results
        let results = prepare_result(results_vec);
        write_to_terminal_multicolor("stress test complete!").expect("unable to write to terminal");
        // print results
        print_result(results.clone());
        return results;
    }
}

// fn to spawn threads of requests
pub fn spawn_threads(
    config: AppConfig<HttpProtocol>,
    number: u32,
) -> Vec<Map<String, Value>> {
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
) -> Vec<Map<String, Value>> {
    // get end duration of stress test loop
    let mut end_duration = config.input_config.protocol.duration.clone();
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
pub fn generate_request<'a>(config: HttpProtocol) -> Map<String, Value> {
    let mut response_map: Map<String, Value> = Map::new();
    let client_builder = reqwest::blocking::Client::builder()
        .timeout(config.timeout)
        .build();
    if client_builder.is_ok() {
        let client = client_builder.unwrap();
        // request generate match
        let resp: Result<reqwest::blocking::Response, reqwest::Error> = match config.method {
            HttpMethods::GET => client.get(config.url.as_str()).send(),
            HttpMethods::POST => client.post(config.url.as_str()).json::<Value>(&Value::from(config.body_json.clone())).send(),
            HttpMethods::PUT => client.put(config.url.as_str()).json::<Value>(&Value::from(config.body_json.clone())).send(),
            HttpMethods::DELETE => client.delete(config.url.as_str()).json::<Value>(&Value::from(config.body_json.clone())).send(),
        };

        // include request information
        response_map.insert("request_url".parse().unwrap(), Value::from(config.url));
        response_map.insert("request_method".parse().unwrap(), Value::from(config.method.to_string()));
        response_map.insert("request_data".parse().unwrap(), Value::from(
            config.body_json.clone().into_iter().map(
                |x: (String, Value)| {
                    format!("{} : {}", x.0.as_str(),  x.1.as_str().unwrap())
                }
            ).collect::<Vec<String>>()
        ));
        response_map.insert("request_headers".parse().unwrap(), Value::from(
            config.headers.into_iter().map(
                |x: (String, Value)| {
                    format!("{} : {}", x.0.as_str(), x.1.as_str().unwrap())
                }
            ).collect::<Vec<String>>()
        ));

        if resp.is_ok() {
            let response = resp.unwrap();

            // if status code is successful
            if response.status().is_success() {
                // println!("success = {:?}", response);
                response_map.insert(String::from("status"), Value::from("success"));

            } else {
                // println!("error = {:?}", response);
                response_map.insert(String::from("status"), Value::from("failure"));
            }

           response_map.insert(
               "code".parse().unwrap(),
               Value::from(response.status().to_string()));

            response_map.insert(
                "headers".parse().unwrap(),
                Value::from(
                        response.headers().into_iter().map(
                           |x: (&HeaderName, &HeaderValue)|  {
                               format!("{} : {}", x.0.as_str(), x.1.to_str().unwrap())
                           }
                        ).collect::<Vec<String>>()
                ),
            );

            // handle response data based on headers
            if response.headers().get("content-type").unwrap() == "application/json" {
                response_map.insert(
                    "data".parse().unwrap(),
                     Value::from(response.json::<Value>().unwrap()),
                );
            } else {
                response_map.insert(
                    "data".parse().unwrap(),
                    Value::from(response.text().unwrap()),
                );
            }
        } else {
            response_map.insert("status".parse().unwrap(), Value::from("failure"));
            let response = resp.unwrap_err();
            // println!("status = {:?}", response);
            response_map.insert(
                "data".parse().unwrap(),
                Value::from(response.to_string()),
            );
        }
    }

    return response_map;
}

// fn to parse received vector of hashmap into final result hashmap
pub fn prepare_result<'a>(
    result_info: Vec<Map<String, Value>>,
) -> Map<String, Value> {
    let mut success_count: f64 = 0.0;
    let mut failed_count: f64 = 0.0;
    let mut percentage_successful: f64 = 0.0;

    for i in result_info.clone() {
        if i["status"] == String::from("success") {
            success_count += 1.0;
        }

        if i["status"] == String::from("failure") {
            failed_count += 1.0
        }

        percentage_successful = (success_count.clone() / (success_count.clone() + failed_count.clone())) * 100.0
    }

    let mut results: Map<String, Value> = Map::new();
    results.insert("Successful Requests".parse().unwrap(), Value::from(success_count));
    results.insert("Failed Requests".parse().unwrap(), Value::from(failed_count));
    results.insert("Percentage Successful".parse().unwrap(), Value::from(percentage_successful.round()));
    results.insert("Total Requests".parse().unwrap(), Value::from(result_info.len() as f64));
    results.insert("Requests".parse().unwrap(),
                   Value::from(result_info.into_iter().map(
                       |x: Map<String, Value>| {
                           Value::from(x)
                       }
                   ).collect::<Vec<Value>>())
    );
    return results;
}

// fn to print final result hashmap
pub fn print_result(results: Map<String, Value>) {
    write_to_terminal_multicolor("Results:").expect("unable to write to terminal");
    // print each entry of the result
    for (title, value) in &results {
        write_to_terminal_multicolor(format!("{t}: {v}", t = title, v = value).as_ref())
            .expect("unable to write to terminal");
    }
}
