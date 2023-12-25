use crate::cli::ExecutionCommand;
use crate::engine::http::{HttpRequest, Method};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

pub struct Attack {
    http_request: HttpRequest,
    run_count: u32,
    concurrent_calls: u8,
}

pub enum AttackErr {
    RequestErr(String),
    RequestExecutionErr(String),
}

pub struct AttackResult {
    response: String,
    time_taken: Duration,
}
impl AttackResult {
    pub fn print(&self) {
        println!("response: {}", self.response);
        println!("time_taken: {} mili secs", self.time_taken.as_millis());
    }
    pub fn encode(&self) -> String {
        format!(
            "response: \r\n{}\r\ntime_taken: {}\r\n",
            self.response,
            self.time_taken.as_millis()
        )
    }
}

impl ToString for AttackErr {
    fn to_string(&self) -> String {
        match self {
            AttackErr::RequestErr(s) => {
                format!("Attack Err msg: failed to create request err: {}", s)
            }

            AttackErr::RequestExecutionErr(s) => {
                format!("Attack Err msg: failed to execute request err: {}", s)
            }
        }
    }
}

impl Attack {
    pub fn new(command: &ExecutionCommand, url: &str) -> Result<Attack, AttackErr> {
        let req = match HttpRequest::new(
            Method::from_str(command.method.as_str()),
            url,
            command.headers.clone(),
            command.params.clone(),
            None,
        ) {
            Ok(r) => r,
            Err(e) => {
                return Err(AttackErr::RequestErr(format!(
                    "failed to create request err:{}",
                    e.to_string()
                )))
            }
        };

        return Ok(Attack {
            http_request: req,
            run_count: command.try_count,
            concurrent_calls: command.concurrent_request,
        });
    }

    pub fn run(self) {
        for _ in 0..self.run_count {
            match self.http_request.execute() {
                Ok(res) => {
                    let res_str = res.into_string().expect("failed to translare into string");
                    println!("res: {}", res_str);
                }
                Err(e) => {
                    println!("ok failed: {}", e.to_string().as_str());
                }
            };
        }
    }

    pub fn run_c(&self) -> Result<Receiver<AttackResult>, String> {
        let (tx, rx) = mpsc::channel();
        let tx_ref = &tx;
        for _ in 0..self.concurrent_calls {
            let req = self.http_request.clone();
            let run_count = self.run_count;
            let cloned_tx = tx_ref.clone();
            thread::spawn(move || {
                for _ in 0..run_count {
                    let start = Instant::now();
                    let req_execution = req.execute();
                    let end = Instant::now().duration_since(start);

                    match req_execution {
                        Ok(res) => {
                            match res.into_string() {
                                Ok(rsp) => {
                                    cloned_tx
                                        .send(AttackResult {
                                            response: rsp,
                                            time_taken: end,
                                        })
                                        .unwrap();
                                }

                                Err(e) => {
                                    //TODO: handle error here
                                    println!("cant convert response into string {}", e.to_string());
                                    return;
                                }
                            };
                        }
                        Err(e) => {
                            //TODO: handle error here
                            println!("ok failed: {}", e.to_string().as_str());
                        }
                    };
                }
            });
        }
        Ok(rx)
    }
}
