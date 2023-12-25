use std::{
    fs::{self, File},
    io::Write,
    time::Duration,
};

use chrono::{format::format, DateTime, Local};

struct TestInstance {
    test_name: String,
    test_time: DateTime<Local>,
}
pub struct Call {
    timestamp: DateTime<Local>,
    command: String,
    request: String,
    result: String,
}

impl TestInstance {
    fn encode(&self) -> String {
        format!(
            "test_name: {}\r\ntest_time: {}\r\n",
            self.test_name, self.test_time
        )
    }
}

impl Call {
    pub fn new(
        timestamp: DateTime<Local>,
        command: String,
        request: String,
        result: String,
    ) -> Call {
        Call {
            timestamp,
            command,
            request,
            result,
        }
    }

    fn encode(&self) -> String {
        format!(
            "{}\t {}\r\n{}\r\n{}\r\n",
            self.timestamp.to_string(),
            self.command,
            self.request,
            self.result
        )
    }
}
pub struct GokuIO {
    log_file: File,
    test_name: String,
}

impl GokuIO {
    pub fn new(
        test_name: &str,
        fs_path: &str,
        test_time: DateTime<Local>,
    ) -> Result<GokuIO, String> {
        let path = format!("{}/{}.goku", fs_path, test_name);

        let mut file = match fs::File::create(path.as_str()) {
            Ok(f) => f,
            Err(e) => {
                println!("error {}", e.to_string());
                return Err(e.to_string());
            }
        };

        let stream = format!("test_name: {}\r\ntest_time: {}\r\n", test_name, test_time);

        file.write(stream.as_bytes());

        Ok(GokuIO {
            log_file: file,
            test_name: test_name.to_string(),
        })
    }

    pub fn write(&mut self, call: &Call) {
        let stream = call.encode();
        self.log_file.write(stream.as_bytes()).unwrap();
        self.log_file.sync_all().unwrap();
    }
}
