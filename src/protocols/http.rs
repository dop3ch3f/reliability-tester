// codes responsible for the handling of http protocol

use std::collections::HashMap;
use std::time::Duration;
use crate::configs::HttpMethods;

// struct for when protocol is Http
#[derive(Clone)]
pub struct HttpProtocol {
    pub method: HttpMethods,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout: Duration,
}

impl HttpProtocol {
    pub fn new(
        method: HttpMethods,
        url: &str,
        headers: HashMap<String, String>,
        timeout: Duration,
    ) -> HttpProtocol {
        HttpProtocol {
            method,
            url: String::from(url),
            headers,
            timeout,
        }
    }
}

