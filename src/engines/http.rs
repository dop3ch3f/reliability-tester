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
use serde_json::{Value};

use crate::configs::{AppConfig, Callback, HttpMethods, RequestStatus};
use crate::protocols::http::HttpProtocol;
use std::collections::hash_map::RandomState;
use crate::util::write_to_terminal_multicolor;


pub fn generate_request (config: HttpProtocol) -> HashMap<&'static str, String> {
    let mut response_map: HashMap<&str, String> = HashMap::new();
    let client = reqwest::blocking::Client::builder()
        .timeout(config.timeout)
        .build()
        .unwrap();
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
            response_map.insert("data", response.text().expect("Unable to convert to string").clone());
        } else {
            // println!("error = {:?}", response);
            response_map.insert("status", "failure".parse().unwrap());
            response_map.insert("data", response.text().expect("Unable to convert to string").clone());
        }
    } else {
        let response = resp.unwrap_err();
        // println!("status = {:?}", response);
        response_map.insert("status", "failure".parse().unwrap());
        response_map.insert("data", String::from(response.status().unwrap().as_str().clone()));
    }

    return response_map;
}

pub fn spawn_threads (config:AppConfig<HttpProtocol>, number: u32 ) -> Vec<HashMap<&'static str, String, RandomState>> {
    let result = Arc::new(Mutex::new(Vec::new()));
    let wg = WaitGroup::new();

    for i in 0..number {
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

    // let output = *result.clone().lock().unwrap().clone();
    // return Vec::from_iter(&output);

}

pub fn prepare_result<'a> (result_info: Vec<HashMap<&str, String, RandomState>>) ->  HashMap<&'a str, f64> {
    let mut success_count:f64 = 0.0;
    let mut failed_count:f64 = 0.0;
    let mut percentage_successful: f64 = 0.0;

    for i in result_info.clone() {
        if i[&"status"] == String::from("success") {
            success_count+=1.0;
        }

        if i[&"status"] == String::from("failure") {
            failed_count+=1.0
        }
        // match i.unwrap().get("status").unwrap() {
        //     *"success".borrow() => {
        //         success_count+=1;
        //     }
        //     RequestStatus::FAILURE => {
        //         failed_count+=1;
        //     },
        //     _ => {
        //         failed_count+=1;
        //     }
        // }

        percentage_successful = (success_count / (success_count + failed_count)) * 100.0
    }

    let mut results = HashMap::new();
    results.insert("Successful Requests:", success_count);
    results.insert("Failed Requests:", failed_count);
    results.insert("Percentage Successful:", percentage_successful);
    results.insert("Total Requests:", result_info.len() as f64);
    return results;
}


pub struct HttpEngine {}

impl HttpEngine {
    // pub fn load_test () ->  HashMap<String, Box<dyn std::error::Error>> {}
    pub fn load_test (config: AppConfig<HttpProtocol>) {
        write_to_terminal_multicolor("load test initiated!");
        let results_vec = spawn_threads(config.clone(), config.process_config.hits.clone() as u32);
        let results = prepare_result(results_vec);
        write_to_terminal_multicolor("load test complete!");
        write_to_terminal_multicolor("Results:");
        for (title, value) in &results {
            write_to_terminal_multicolor(format!("{t}: {v}", t = title, v = value).as_ref());
        }

    }
    pub fn stress_test () {}
}