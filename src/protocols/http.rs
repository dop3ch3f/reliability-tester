// codes responsible for the handling of http protocol

use crate::configs::HttpMethods;
use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// struct for when protocol is Http
#[derive(Clone, Deserialize, Serialize)]
pub struct HttpProtocol {
    pub method: HttpMethods,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body_json: HashMap<String, Value>,
    pub timeout: Duration,
    pub hits: i32,
    pub duration: Duration,
}

impl HttpProtocol {
    pub fn new(
        method: HttpMethods,
        url: &str,
        headers: HashMap<String, String>,
        body_json: HashMap<String, Value>,
        timeout: Duration,
        hits: i32,
        duration: Duration,
    ) -> HttpProtocol {
        HttpProtocol {
            method,
            url: String::from(url),
            headers,
            body_json,
            timeout,
            hits,
            duration,
        }
    }
}

// struct for when protocol is Http and multiple configs are available
#[derive(Clone, Deserialize, Serialize)]
pub struct HttpProtocolMulti{
    pub requests: Vec<HttpProtocol>,
}

impl HttpProtocolMulti {
    // pub fn new(requests: Vec<HttpProtocol>) -> HttpProtocolMulti {
    //     HttpProtocolMulti{
    //         requests
    //     }
    // }
}
