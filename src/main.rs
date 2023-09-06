// Reliability Tester

extern crate clap;

mod configs;
mod engines;
mod ignition;
mod outputs;
mod protocols;
mod util;
mod frontend;


// use engines::http::http_engine;

use crate::ignition::console::ignite_console;
use crate::util::write_to_terminal_multicolor;
use clap::Parser;




use crate::ignition::server::ignite_web_server;
use crate::ignition::desktop::ignite_desktop;
use crate::ignition::web::ignite_web_app;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
}

async fn ignition() -> std::io::Result<()> {

    let args = Args::parse();
    write_to_terminal_multicolor(
        "Reliability Tester\nIfeanyi Ibekie <ifeanyi.ibekie@gmail.com>\nTest the reliability of your services\nWhat flavor would you like to run: (cli, api, web, desktop)\n"
    ).expect("TODO: panic message");

    match String::from(args.mode).as_str() {
        "api" => {
            ignite_web_server().await?
        }
        "web" => {
            ignite_web_app()
        }
        "gui" => {
            ignite_desktop()
        }
        "cli" => {
            // Todo: Add support for file input in cli mode
            ignite_console().expect("TODO: panic message");
        }
        _ => {
            write_to_terminal_multicolor(
                "No mode chosen exiting... (try running --help to view list of available modes)",
            ).expect("TODO: panic message");
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    write_to_terminal_multicolor("Welcome to The Reliability Tester").expect("TODO: panic message");
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
