pub mod cli;
pub mod engine;
pub mod goku;
pub mod goku_data_io;
use chrono::Local;
use cli::ExecutionCommand;
use goku::{Attack, AttackResult, ExecutionSet};
use goku_data_io::data_io::{self, Call, GokuIO};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    io::{self, Read, Write},
    sync::mpsc::Receiver,
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
                let mut path: String = test_name.clone();
                path.push_str(".logs.json");
                let mut file = fs::File::create(path).unwrap();

                file.write(execution_set.to_string().as_bytes()).unwrap();
            }
        }

        "report" => {
            let mut test_name = match args[2].to_str() {
                Some(tn) => tn.to_string(),
                None => {
                    println!("test name is required");
                    return;
                }
            };
            test_name.push_str(".logs.json");
            let mut file = fs::File::open(test_name).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let es = ExecutionSet::from_string(&content);
            let report = es.get_report();

            let mut report_path = match args[2].to_str() {
                Some(tn) => tn.to_string(),
                None => {
                    println!("test name is required");
                    return;
                }
            };

            report_path.push_str(".report.json");
            let mut report_file = fs::File::create(report_path).unwrap();
            let mut report_content = report.to_string();
            report_file.write(report_content.as_bytes()).unwrap();
        }

        _ => run_help(),
    }
}
