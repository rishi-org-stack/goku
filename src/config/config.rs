use std::env;

pub struct Config {
    pub report_dir: String,
    pub api_call_log_dir: String,
    pub app_logs_file: String,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let report_dir = env::var("REPORT_DIR").unwrap_or("/home/rishi/goku/".to_string());

        let api_call_log_dir =
            env::var("API_CALL_LOG_DIR").unwrap_or("/home/rishi/goku/".to_string());

        let api_logs_file =
            env::var("API_LOGS_FILE").unwrap_or("/home/rishi/goku/logs.goku".to_string());

        Ok(Config {
            report_dir: report_dir,
            api_call_log_dir: api_call_log_dir,
            app_logs_file: api_logs_file,
        })
    }
}
