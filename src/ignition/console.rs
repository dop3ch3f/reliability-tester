extern crate clap;

use crate::configs::{AppConfig, HttpMethods, InputConfig, OutputConfig, ProcessConfig};
use crate::engines::http::HttpEngine;
use crate::protocols::http::HttpProtocol;
use crate::util::write_to_terminal_multicolor;
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::io;
use std::time::Duration;

fn setup_http() -> HttpProtocol {
    let mut method = String::new();
    let mut url = String::new();
    write_to_terminal_multicolor("Input the method ?  [get/post/put/delete]");
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
    write_to_terminal_multicolor("Input the url ? [any@any.any]");
    io::stdin()
        .read_line(&mut url)
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
        HashMap::new(),
        Duration::from_secs(120),
    );
    http_config
}

fn setup_process(stress_type: &str) -> ProcessConfig {
    let mut hits = String::new();

    let mut duration = String::new();

    write_to_terminal_multicolor(&*format!("{} mode", stress_type));

    write_to_terminal_multicolor("Input the number of hits?  [0-9999]");

    io::stdin()
        .read_line(&mut hits)
        .expect("Failed to read line");

    if stress_type.lines().next().unwrap() == "stress" {
        write_to_terminal_multicolor("Input the duration of the stress test in seconds?  [0-9999]");

        io::stdin()
            .read_line(&mut duration)
            .expect("Failed to read line");
    }

    let process_config: ProcessConfig = ProcessConfig {
        hits: hits.lines().next().unwrap().parse::<i32>().unwrap(),

        is_load_test: if stress_type.lines().next().unwrap() == "load" { true } else { false },

        is_stress_test: if stress_type.lines().next().unwrap() == "stress" { true } else { false },

        duration: if stress_type.lines().next().unwrap() == "stress" {
            Duration::from_secs(duration.lines().next().unwrap().parse::<u64>().unwrap())
        } else {
            Duration::from_secs(120)
        },
    };
    process_config
}

pub fn ignite_console() -> Result<(), Box<dyn std::error::Error>> {
    write_to_terminal_multicolor("Preparing Console Mode...");
    write_to_terminal_multicolor(
        "Do you want to perform a stress test or load test?  [stress/load]",
    );

    let mut stress_type = String::new();
    io::stdin()
        .read_line(&mut stress_type)
        .expect("Failed to read line");

    let http_config = setup_http();

    let process_config = setup_process(stress_type.as_str());

    // trigger the http engine
    HttpEngine::new(AppConfig {
        input_config: InputConfig {
            protocol: http_config,
        },
        process_config,
        output_config: OutputConfig {
            logging: true,
            console: true,
            file: true,
        },
    });

    Ok(())
}
