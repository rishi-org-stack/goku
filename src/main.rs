pub mod cli;
pub mod config;
pub mod engine;
pub mod goku;
use cli::ExecutionCommand;
use config::config::Config;
use goku::{Attack, ExecutionSet};
use std::{
    env,
    ffi::OsString,
    fs,
    io::{Read, Write},
};

fn run_help() {
    println!("help func");
}
fn main() {
    let args: Vec<OsString> = env::args().map(OsString::from).collect();
    if args.len() < 3 {
        run_help();
        return;
    }

    let execution_type: &str = match args[1].to_str() {
        Some(first_arg) => first_arg,
        None => {
            println!("no arg found");
            return;
        }
    };

    let conf = Config::new().unwrap();

    match execution_type {
        "attack" => {
            let url = match args[2].to_str() {
                Some(u) => u,
                None => {
                    println!("url is required");
                    return;
                }
            };

            let attack_args: Vec<OsString> = args[2..].to_vec();
            let execution_command: ExecutionCommand = ExecutionCommand::new(&attack_args);

            let attack = match Attack::new(&execution_command, url) {
                Ok(a) => a,

                Err(e) => {
                    println!("failed to attack on {} err: {}", url, e.to_string());
                    return;
                }
            };

            let execution_set: ExecutionSet = match attack.run_c() {
                Ok(result) => result,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };

            if let Some(test_name) = execution_command.test_name {
                let mut actual_path = conf.api_call_log_dir;

                let mut path: String = test_name.clone();
                path.push_str(".logs.json");

                actual_path.push_str(&path);
                let mut file = fs::File::create(actual_path).unwrap();

                file.write(execution_set.to_string().as_bytes()).unwrap();
            }
        }

        "report" => {
            let mut actual_path = conf.api_call_log_dir;

            let mut test_name = match args[2].to_str() {
                Some(tn) => tn.to_string(),
                None => {
                    println!("test name is required");
                    return;
                }
            };
            test_name.push_str(".logs.json");
            actual_path.push_str(&test_name);

            let mut file = fs::File::open(actual_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let es = ExecutionSet::from_string(&content);
            let report = es.get_report();

            let mut actual_report_path = conf.report_dir;

            let mut report_path = match args[2].to_str() {
                Some(tn) => tn.to_string(),
                None => {
                    println!("test name is required");
                    return;
                }
            };

            report_path.push_str(".report.json");

            actual_report_path.push_str(&report_path);

            let mut report_file = fs::File::create(actual_report_path).unwrap();
            let report_content = report.to_string();
            report_file.write(report_content.as_bytes()).unwrap();
        }

        _ => run_help(),
    }
}
