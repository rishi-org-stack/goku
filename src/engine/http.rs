use serde_json::{self, Map, Value};
use std::time::Duration;
use ureq::{Agent, Error, Request, Response};
use url::Url;

#[derive(Copy, Clone)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_str(method: &str) -> Method {
        match method.to_lowercase().as_str() {
            "get" => return Method::GET,
            "post" => return Method::POST,
            _ => {
                println!("--> unknown method found setting it basck to get");
                return Method::GET;
            }
        }
    }
}

#[derive(Clone)]
pub struct HttpRequest {
    url: Url,
    client: Agent,
    method: Method,
    headers: Option<Map<String, Value>>,
    params: Option<Map<String, Value>>,
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
        headers_str: Option<String>,
        params_str: Option<String>,
        req_conf: Option<Config>,
    ) -> Result<HttpRequest, String> {
        let url: Url = match addr.parse() {
            Ok(u) => u,
            Err(e) => return Err(e.to_string()),
        };

        let mut headers: Option<Map<String, Value>> = None;
        match headers_str {
            Some(hs) => {
                headers = match serde_json::from_str::<Value>(hs.as_str()) {
                    Ok(Value::Object(map)) => Some(map),
                    _ => None,
                };
            }
            None => (),
        };

        let mut params: Option<Map<String, Value>> = None;
        match params_str {
            Some(ps) => {
                params = match serde_json::from_str::<Value>(ps.as_str()) {
                    Ok(Value::Object(map)) => Some(map),
                    _ => None,
                };
            }

            None => (),
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

        Ok(HttpRequest {
            client,
            method: method_enum,
            url,
            params,
            headers,
        })
    }

    pub fn execute(&self) -> Result<HttpResponse, Error> {
        let mut method: &str = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
        };

        let mut request: Request = self.client.request(method, self.url.as_str());

        if let Some(p) = &self.params {
            for (k, v) in p {
                request = request.query(k.as_str(), v.as_str().unwrap());
            }
        }

        if let Some(h) = &self.headers {
            for (k, v) in h {
                request = request.set(k.as_str(), v.as_str().unwrap());
            }
        }

        let response = request.call().expect("failed to send request");
        Ok(response)
    }
}
