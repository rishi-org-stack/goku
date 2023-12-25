pub mod cli;
pub mod engine;
pub mod goku;
pub mod goku_data_io;
use chrono::Local;
use cli::ExecutionCommand;
use goku::{Attack, AttackResult};
use goku_data_io::data_io::{self, Call, GokuIO};
use std::{
    env,
    ffi::{OsStr, OsString},
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

            let attack_result_receiver: Receiver<AttackResult> = match attack.run_c() {
                Ok(result) => result,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };
            let now = Local::now();
            let mut gio: GokuIO = match data_io::GokuIO::new("logo", ".", now) {
                Ok(gio) => gio,
                Err(e) => {
                    println!("init gio error: {}", e.to_string());
                    return;
                }
            };
            if let Some(_lf) = execution_command.log_file {
                let test_name = match &execution_command.test_name {
                    Some(t) => t.clone(),
                    None => String::from("value"),
                };

                gio = match data_io::GokuIO::new(test_name.as_str(), ".", now) {
                    Ok(gio) => gio,
                    Err(e) => {
                        println!("init gio error: {}", e.to_string());
                        return;
                    }
                };
            }

            for result in attack_result_receiver {
                let call = Call::new(
                    now,
                    attack_args
                        .join(OsStr::new(" "))
                        .to_str()
                        .unwrap()
                        .to_string(),
                    "request".to_string(),
                    result.encode(),
                );
                if execution_command.verbose {
                    result.print();
                }
                if let Some(_lf) = execution_command.log_file {
                    gio.write(&call)
                }
            }
        }

        "report" => {
            println!("reporting command")
        }

        _ => run_help(),
    }
}
