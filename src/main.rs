// Reliability Tester

extern crate clap;

mod configs;
mod engines;
mod ignition;
mod outputs;
mod protocols;
mod util;

use configs::{AppConfig, HttpMethods, InputConfig, OutputConfig, ProcessConfig};
// use engines::http::http_engine;
use crate::engines::http::HttpEngine;
use crate::ignition::console::ignite_console;
use crate::util::write_to_terminal_multicolor;
use clap::{App, Arg, SubCommand};
use protocols::http::HttpProtocol;
use std::collections::HashMap;
use std::io;
use std::time::Duration;
use crate::ignition::server::ignite_web_server;
use crate::ignition::desktop::ignite_desktop;

async fn ignition() -> std::io::Result<()> {
    let matches = App::new("Reliability Tester")
        .version("1.0")
        .author("Ifeanyi Ibekie <ifeanyi.ibekie@gmail.com>")
        .about("Test the reliability of your microservices")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .value_name("mode")
                .help("What flavor would you like to run: (cli, api, web, desktop)")
                .takes_value(true),
        )
        .get_matches();

    let launch_type = matches.value_of("mode");
    match launch_type {
        Some("api") => {
            ignite_web_server().await?
        }
        Some("web") => {}
        Some("gui") => {
            ignite_desktop()
        }
        Some("cli") => {
            // Todo: Add support for file input in cli mode
            ignite_console();
        }
        _ => {
            write_to_terminal_multicolor(
                "No mode chosen exiting... (try running --help to view list of available modes)",
            );
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    write_to_terminal_multicolor("Welcome to The Reliability Tester");
    // kick start the application by running ignition
    ignition().await

    // // generate test http config
    // let http_config: HttpProtocol = HttpProtocol::new(
    //     HttpMethods::GET,
    //     "https://staging.getmedialit.com",
    //     HashMap::new(),
    //     Duration::from_secs(120),
    // );
    // // generate input config
    // let input_config: InputConfig<HttpProtocol> = InputConfig {
    //     protocol: http_config,
    // };
    // // generate process config
    // let process_config: ProcessConfig = ProcessConfig {
    //     hits: 20,
    //     is_load_test: false,
    //     is_stress_test: true,
    //     duration: Duration::from_secs(120),
    // };
    // // generate output config
    // let output_config: OutputConfig = OutputConfig {
    //     logging: true,
    //     console: true,
    //     file: true,
    // };
    // // generate operation config
    // let config: AppConfig<HttpProtocol> = AppConfig {
    //     input_config,
    //     process_config,
    //     output_config,
    // };
    //
    // // trigger the http engine
    // HttpEngine::new(config);

    //Ok(())
}
