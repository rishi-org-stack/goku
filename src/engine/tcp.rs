use http::request::Builder;
use http::HeaderValue;
use http::Method;
use http::Request;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

struct HttpRequest {
    request: String,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl HttpRequest {
    fn new(
        method: Method,
        addr: &str,
        header: &str,
        params: &str,
        body: Option<String>,
    ) -> HttpRequest {
        let mut updated_uri = String::from(addr);

        if params.len() > 0 {
            let params_hash_map: HashMap<String, String> =
                serde_json::from_str(params).expect("invalid params");

            updated_uri = format!("{}?", addr);
            for kv in params_hash_map {
                updated_uri = format!("{}{}={}&", updated_uri, kv.0, kv.1);
            }
            updated_uri = String::from(updated_uri.trim_end_matches("&"));
        }

        let mut req_builder = Builder::new().method(method).uri(updated_uri);
        if header.len() > 0 {
            let headers_hash_map: HashMap<String, String> =
                serde_json::from_str(header).expect("invalid header format");

            for kv in headers_hash_map {
                req_builder = req_builder.header(kv.0, kv.1);
            }
        }

        let req: Request<()> = req_builder.body(()).expect("failed to create request");

        let mut request_str = format!(
            "{} {} {:?}\r\n{}: {}\r\n{}: {}\r\n{}\r\n",
            req.method(),
            req.uri().path(),
            req.version(),
            "Content-Type",
            "application/json",
            "Host",
            "jsonplaceholder.typicode.com",
            req.headers()
                .iter()
                .map(|(k, v)| {
                    println!("hhh{}", k);
                    format!("{}: {}", k, v.to_str().unwrap())
                })
                .collect::<Vec<_>>()
                .join("\r\n")
        );
        if body != None {
            request_str.push_str(body.expect("unexpected body").as_str());
        }

        HttpRequest {
            request: request_str,
        }
    }
}

fn dd() {
    // Create the HTTP request
    let http_req = HttpRequest::new(
        Method::POST,
        "https://jsonplaceholder.typicode.com/posts",
        "",
        "",
        Some(String::from(
            r#"{"title":"foolish","body":"bar", userId:1}"#,
        )),
    );

    // Convert the request to a string

    println!("{}", http_req.request);
    // Connect to the server
    let mut stream = TcpStream::connect("jsonplaceholder.typicode.com:443").unwrap();

    // Write the HTTP request
    stream.write_all(http_req.request.as_bytes()).unwrap();

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("{}", response);
}
