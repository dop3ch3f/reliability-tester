use std::time::Duration;
use serde::{Deserialize, Serialize};


// struct for application configuration
#[derive(Copy, Clone,Deserialize, Serialize)]
pub struct AppConfig<I> {
    pub input_config: InputConfig<I>,
    pub process_config: ProcessConfig,
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
    pub hits: i32,
    pub is_stress_test: bool,
    pub is_load_test: bool,
    pub duration: Duration,
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

pub enum RequestStatus {
    SUCCESS,
    FAILURE,
}

pub struct GlobalServerState {
    pub app_name: String,
}
