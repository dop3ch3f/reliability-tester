// codes responsible for the handling of http protocol

use crate::configs::HttpMethods;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// struct for when protocol is Http
#[derive(Clone, Deserialize, Serialize)]
pub struct HttpProtocol {
    pub method: HttpMethods,
    pub url: String,
    pub headers: Map<String, Value>,
    pub body_json: Map<String, Value>,
    pub timeout: Duration,
    pub hits: i32,
    pub duration: Duration,
}

impl HttpProtocol {
    pub fn new(
        method: HttpMethods,
        url: &str,
        headers: Map<String, Value>,
        body_json: Map<String, Value>,
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
