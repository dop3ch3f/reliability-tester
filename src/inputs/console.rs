// codes responsible for reading from console and generating the right application config
extern crate clap;
use crate::configs::{AppConfig, InputConfig, OutputConfig, ProcessConfig};
use clap::{App, Arg, SubCommand};

pub fn process_console_inputs() {
    let matches = App::new("Reliability Tester")
        .version("1.0")
        .author("Ifeanyi Ibekie <ifeanyi.ibekie@gmail.com>")
        .about("Test the reliability of your microservices")
        // for http only
        .arg(
            Arg::with_name("http")
                .short("ht")
                .long("http")
                .value_name("method[get,post,put,delete]|")
                .help("Sets all the config for a http test")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("process configuration")
                .short("p")
                .long("process-config")
                .value_name("hits[int]|is_stress_test[bool]|duration[int]"),
        )
        .arg(
            Arg::with_name("input configuration")
                .short("i")
                .long("input-config")
                .value_name("method[get,post,put,delete]")
                .help("Sets the custom")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();
}
