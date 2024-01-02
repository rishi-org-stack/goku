use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::cli::ExecutionCommand;
use crate::engine::http::{HttpRequest, Method};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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

    // pub fn run(http_req: HttpRequest) {
    //     thread::spawn(move || {
    //         http_req.execute();
    //     });
    // }

    pub fn run_c(&self) -> Result<ExecutionSet, &'static str> {
        let mut call_list: CallMap = HashMap::new();
        let (tx, rx) = mpsc::channel::<(u128, Execution)>();

        for _ in 0..self.run_count {
            let mut threads = vec![];
            for _ in 0..self.concurrent_calls {
                let http_req = self.http_request.clone();
                let cloned_tx = tx.clone();

                let t = thread::spawn(move || {
                    let start: SystemTime = SystemTime::now();
                    let exec_result = http_req.execute();
                    let time_taken: u128 =
                        SystemTime::now().duration_since(start).unwrap().as_millis();

                    match exec_result {
                        Ok(res) => {
                            // let resp_body = &res.into_string().unwrap_or("none".to_string());
                            cloned_tx
                                .send((
                                    start.duration_since(UNIX_EPOCH).unwrap().as_secs() as u128,
                                    Execution {
                                        time_taken,
                                        status: res.status(),
                                        result: "result",
                                    },
                                ))
                                .unwrap();
                        }
                        Err(e) => {
                            println!("err :{}", e.to_string().as_str());
                        }
                    };
                });

                threads.push(t);
            }

            for child_t in threads {
                child_t.join().unwrap();
            }

            for exec in rx.recv() {
                call_list.insert(exec.0, exec.1);
            }
        }
        Ok(ExecutionSet::new("req", call_list))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Execution<'a> {
    time_taken: u128,
    status: u16,
    result: &'a str,
}
type CallMap<'a> = HashMap<u128, Execution<'a>>;
#[derive(Serialize, Deserialize)]
pub struct ExecutionSet<'a> {
    request: &'a str,
    call_list: CallMap<'a>,
}

impl<'a> ExecutionSet<'a> {
    pub fn new(request: &'a str, call_list: CallMap<'a>) -> ExecutionSet<'a> {
        ExecutionSet { request, call_list }
    }
}

pub fn ok_print() {
    let mut mp: HashMap<u128, Execution> = HashMap::new();
    mp.insert(
        1,
        Execution {
            status: 200,
            time_taken: 65,
            result: "a",
        },
    );
    mp.insert(
        2,
        Execution {
            status: 404,
            time_taken: 66,
            result: "b",
        },
    );
    let s = ExecutionSet::new("req 1", mp);
    let serlialized = serde_json::to_string_pretty(&s).unwrap();
    println!("data: {}", serlialized)
}
