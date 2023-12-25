use std::ffi::OsString;

use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ExecutionCommand {
    /// method of api call
    #[arg(short, long, default_value_t=String::from("GET"))]
    pub method: String,

    /// Headers for api call (optional)
    #[arg(long, required = false)]
    pub headers: Option<String>,

    /// Params for api call
    #[arg(long, required = false)]
    pub params: Option<String>,

    /// Body for api call
    #[arg(long, required = false)]
    pub body: Option<String>,

    /// file to log responses for api call
    #[arg(long, short, required = false)]
    pub log_file: Option<bool>,

    /// file to log responses for api call
    #[arg(long, required = false)]
    pub test_name: Option<String>,

    /// Try_count number of times to call
    #[arg(long, short, default_value_t = 1)]
    pub try_count: u32,

    // /// Concurrent_request number of concurrent request to process at once
    #[arg(long, short, required = false)]
    pub concurrent_request: u8,
    /// Additional optional flag to display
    #[arg(short, long, required = false)]
    pub verbose: bool,
}

impl ExecutionCommand {
    pub fn new(it: &Vec<OsString>) -> ExecutionCommand {
        return ExecutionCommand::parse_from(it);
    }
}
//goku attack 'url' -method='GET|POST|PUT|DELETE' -body='{...data}' --tc=2 -v
