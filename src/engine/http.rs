use std::{collections::HashMap, time::Duration};

use serde_json;
use ureq::{Agent, Error, Header, Request, Response};
use url::{ParseError, Url};

pub enum Method {
    GET,
    POST,
}
pub struct HttpRequest {
    base: Url,
    client: Agent,
    method: Method,
    header: HashMap<&'static str, &'static str>,
    params: HashMap<&'static str, &'static str>,
}

type HttpResponse = Response;
pub struct Config {
    read_timeout_in_s: u64,
    write_timeout_in_s: u64,
}
impl HttpRequest {
    pub fn new(
        method_enum: Method,
        addr: &str,
        headers_str: &'static str,
        params_str: &'static str,
        req_conf: Option<Config>,
    ) -> Option<HttpRequest> {
        let url: Url = addr
            .parse()
            .map_err(|e: ParseError| {
                let err_msg = format!("invalid url err: {}", e.to_string());
                eprintln!("{}", err_msg);
            })
            .unwrap();

        let mut headers: HashMap<&str, &str> = HashMap::new();
        if headers_str.len() > 0 {
            headers = serde_json::from_str(headers_str).expect("invalid header format");
        }

        let mut params: HashMap<&str, &str> = HashMap::new();
        if params_str.len() > 0 {
            params = serde_json::from_str(params_str).expect("invalid param format");
        }

        let mut timeout_read: Duration = Duration::from_secs(5);
        let mut timeout_write: Duration = Duration::from_secs(5);

        if req_conf.is_some() {
            let cnf = req_conf.unwrap();
            timeout_read = Duration::from_secs(cnf.read_timeout_in_s);
            timeout_write = Duration::from_secs(cnf.write_timeout_in_s);
        }

        let client: Agent = ureq::AgentBuilder::new()
            .timeout_read(timeout_read)
            .timeout_write(timeout_write)
            .https_only(true)
            .build();

        Some(HttpRequest {
            client,
            method: method_enum,
            base: url,
            params,
            header: headers,
        })
    }

    pub fn execute(self) -> Result<HttpResponse, Error> {
        let mut method: &str = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
        };

        let mut request: Request = self.client.request(method, self.base.as_str());
        for kv in self.params {
            request = request.query(kv.0, kv.1);
        }
        for kv in self.header {
            request = request.set(kv.0, kv.1);
        }

        let response = request.call().expect("failed to send request");

        Ok(response)
    }
}
