use std::fmt;
use std::time::Duration;
use serde::{Deserialize, Serialize};


// struct for application configuration
#[derive(Copy, Clone,Deserialize, Serialize)]
pub struct AppConfig<I> {
    pub input_config: InputConfig<I>,
    pub output_config: OutputConfig,
}


// struct for data received from input channels, namely : console, http e.t.c
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct InputConfig<I> {
    pub protocol: I,
}

// struct for processing configurations and settings namely global to whatever method there is:  hits, stress_test, duration e.t.c
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct ProcessConfig {

}

// struct for output configurations and settings namely: file,console,logging enabled e.t.c
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    pub console: bool,
    pub logging: bool,
    pub file: bool,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

impl fmt::Display for HttpMethods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethods::GET => write!(f, "get"),
            HttpMethods::POST => write!(f, "post"),
            HttpMethods::PUT => write!(f, "put"),
            HttpMethods::DELETE => write!(f, "delete"),
        }
    }
}

// pub enum RequestStatus {
//     SUCCESS,
//     FAILURE,
// }

pub struct GlobalServerState {
    pub app_name: String,
}
