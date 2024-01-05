use serde::{Deserialize, Serialize};

use crate::cli::ExecutionCommand;
use crate::engine::http::{HttpRequest, Method};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};
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
        let mut call_list: CallMap = Vec::new();
        let (tx, rx) = mpsc::channel::<(u128, Execution)>();
        let mut x: u128 = 0;
        for _ in 0..self.run_count {
            let mut threads = vec![];
            for _ in 0..self.concurrent_calls {
                x += 1;
                let http_req = self.http_request.clone();
                let cloned_tx = tx.clone();

                let t = thread::spawn(move || {
                    let start: SystemTime = SystemTime::now();
                    let exec_result = http_req.execute();
                    let time_taken: u128 =
                        SystemTime::now().duration_since(start).unwrap().as_millis();

                    match exec_result {
                        Ok(res) => {
                            cloned_tx
                                .send((
                                    // start.duration_since(UNIX_EPOCH).unwrap().as_secs() as u128,
                                    x,
                                    Execution {
                                        time_taken,
                                        status: res.status(),
                                        result: "result",
                                    },
                                ))
                                .expect("msg");
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
            for _ in 0..self.concurrent_calls {
                let exec = match rx.recv() {
                    Ok(rs) => rs,
                    Err(e) => {
                        println!("error recieving: {}", e.to_string());
                        break;
                    }
                };

                call_list.push(exec.1);
            }
        }
        Ok(ExecutionSet::new(
            "req",
            call_list,
            (self.run_count * self.concurrent_calls as u32) as u32,
        ))
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Execution<'a> {
    time_taken: u128,
    status: u16,
    result: &'a str,
}
type CallMap<'a> = Vec<Execution<'a>>;
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionSet<'a> {
    total_calls: u32,
    request: &'a str,
    call_list: CallMap<'a>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDistribution {
    count: u32,
    percentage: u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    avg_latency: u128,
    max_latency: u128,
    min_latency: u128,
    code_distribution: HashMap<u16, CodeDistribution>,
    total_calls: u32,
    // concurrent_calls: u32,
}

impl Metrics {
    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
impl<'a> ExecutionSet<'a> {
    pub fn new(request: &'a str, call_list: CallMap<'a>, total_calls: u32) -> ExecutionSet<'a> {
        ExecutionSet {
            request,
            call_list,
            total_calls,
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn from_string(s: &'a str) -> ExecutionSet<'a> {
        let es: ExecutionSet = serde_json::from_str(s).unwrap();
        es
    }

    pub fn get_report(&self) -> Metrics {
        let call_list_count = self.call_list.len();
        self.call_list.iter().fold(
            Metrics {
                avg_latency: 0,
                max_latency: u128::MIN,
                min_latency: u128::MAX,
                code_distribution: HashMap::new(),
                total_calls: self.total_calls,
            },
            |acc, x| {
                let avg_latency = acc.avg_latency + (x.time_taken / call_list_count as u128);
                let max_latency = u128::max(acc.max_latency, x.time_taken);
                let min_latency = u128::min(acc.min_latency, x.time_taken);

                let mut code_distribution = acc.code_distribution;
                code_distribution
                    .entry(x.status)
                    .and_modify(|c| {
                        c.count += 1;
                        c.percentage += (100 / self.total_calls) as u8
                    })
                    .or_insert(CodeDistribution {
                        count: 1,
                        percentage: (100 / self.total_calls) as u8,
                    });

                Metrics {
                    avg_latency,
                    max_latency,
                    min_latency,
                    code_distribution: code_distribution,
                    total_calls: acc.total_calls,
                    // concurrent_calls:
                }
            },
        )
    }
}
