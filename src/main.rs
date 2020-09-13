// Reliability Tester

mod configs;
mod engines;
mod inputs;
mod outputs;
mod protocols;
mod util;

use configs::{AppConfig, InputConfig, OutputConfig, ProcessConfig, HttpMethods};
// use engines::http::http_engine;
use inputs::console::process_console_inputs;
use protocols::http::HttpProtocol;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use crate::engines::http::HttpEngine;
use crate::util::write_to_terminal_multicolor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write_to_terminal_multicolor("Welcome to The Reliability Tester");
    // process all possible inputs
    // process_console_inputs();
    // generate test http config
    let http_config: HttpProtocol = HttpProtocol::new(
        HttpMethods::GET,
        "https://staging.getmedialit.com",
        HashMap::new(),
        Duration::from_secs(120),
    );
    // generate input config
    let input_config: InputConfig<HttpProtocol> = InputConfig {
        protocol: http_config,
    };
    // generate process config
    let process_config: ProcessConfig = ProcessConfig {
        hits: 500,
        is_load_test: true,
        is_stress_test: false,
        duration: Duration::from_secs(120),
    };
    // generate output config
    let output_config: OutputConfig = OutputConfig {
        logging: true,
        console: true,
        file: true,
    };
    // generate operation config
    let config: AppConfig<HttpProtocol> = AppConfig {
        input_config,
        process_config,
        output_config,
    };

    // trigger the http engine
    HttpEngine::load_test(config);

    Ok(())
}
