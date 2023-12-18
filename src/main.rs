pub mod engine;
use engine::http::{HttpRequest, Method};
fn main() {
    let req = HttpRequest::new(
        Method::GET,
        "https://jsonplaceholder.typicode.com/todos/1",
        "",
        "",
        None,
    )
    .unwrap();
    match req.execute() {
        Ok(res) => {
            let resStr = res.into_string().expect("failed to translare into string");
            println!("res: {}", resStr);
        }
        Err(e) => {
            println!("ok failed: {}", e.to_string().as_str());
        }
    };
}
