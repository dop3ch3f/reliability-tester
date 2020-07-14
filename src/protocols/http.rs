// codes responsible for the handling of http protocol

use std::collections::HashMap;
use std::time::Duration;

// struct for when protocol is Http
pub struct HttpProtocol {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout: Duration,
}

impl HttpProtocol {
    pub fn new(
        method: &str,
        url: &str,
        headers: HashMap<String, String>,
        timeout: Duration,
    ) -> HttpProtocol {
        HttpProtocol {
            method: String::from(method),
            url: String::from(url),
            headers,
            timeout,
        }
    }
}
