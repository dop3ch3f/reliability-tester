// Reliability Tester

mod configs;
mod engines;
mod inputs;
mod outputs;
mod protocols;

use configs::{AppConfig, InputConfig, OutputConfig, ProcessConfig};
use engines::http::http_engine;
use protocols::http::HttpProtocol;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate test http config
    let http_config: HttpProtocol = HttpProtocol::new(
        "get",
        "https://www.google.com",
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
        is_stress_test: true,
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

    let config_arc = Arc::new(config);
    let results = http_engine(&config_arc).expect("error occurred here");
    println!("final output: {:?}", results);
    Ok(())
}
